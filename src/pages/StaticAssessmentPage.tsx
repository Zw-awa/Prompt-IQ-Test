import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { Link, useLocation, useNavigate } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import {
  loadNextStaticTask,
  submitStaticPrompt,
  type FinalReportSummary,
  type ScoredTaskSummary,
  type SessionSummary,
  type StaticTaskData,
  type TaskProgress,
} from '../lib/tauri'

type PageStatus =
  | 'idle'
  | 'loading'
  | 'ready'
  | 'submitting'
  | 'scoring'
  | 'completed'
  | 'failed'
  | 'abandoned'

type RouteState = {
  entryMode?: string
  session?: SessionSummary
  initialTaskReady?: boolean
}

type TaskReviewItem = {
  taskId: string
  sequenceNo: number
  title: string
  task: string
  requirements: string[]
  userPrompt: string
  finalScore: number
  executorOutputPreview: string | null
}

function guardRouteState(state: RouteState): string | null {
  if (!state.session?.sessionId) return '缺少 sessionId'
  if (state.entryMode !== 'static') return 'entryMode 不是 static'
  if (state.session.interactionMode !== 'static') return 'interactionMode 不是 static'
  if (!state.session.staticStrategy) return '缺少 staticStrategy'
  if (state.session.status !== 'running') return '会话状态不是 running'
  if (state.initialTaskReady !== true) return 'initialTaskReady 不是 true'
  return null
}

export function StaticAssessmentPage() {
  const navigate = useNavigate()
  const location = useLocation()
  const routeState = (location.state as RouteState | null) ?? {}

  const [status, setStatus] = useState<PageStatus>('idle')
  const [errorMessage, setErrorMessage] = useState('')
  const [errorCode, setErrorCode] = useState('')
  const [currentTask, setCurrentTask] = useState<StaticTaskData | null>(null)
  const [progress, setProgress] = useState<TaskProgress | null>(null)
  const [promptDraft, setPromptDraft] = useState('')
  const [scoredTask, setScoredTask] = useState<ScoredTaskSummary | null>(null)
  const [finalReport, setFinalReport] = useState<FinalReportSummary | null>(null)
  const [taskReviewItems, setTaskReviewItems] = useState<TaskReviewItem[]>([])

  const isAbandoning = useRef(false)

  const guardError = useMemo(() => guardRouteState(routeState), [routeState])

  const session = routeState.session!
  const strategy = session.staticStrategy ?? 'prompt_only'
  const scopeLabel = session.scope === 'full' ? '全面测评' : '趣味速测'
  const sourceLabel = session.questionSource === 'ai_generated' ? 'AI 出题' : '题库抽样'
  const strategyLabel = strategy === 'prompt_only' ? '只评 Prompt' : '执行后再评'

  const summaryText = useMemo(() => {
    if (strategy === 'prompt_only') {
      return '本模式只评估你写出的 Prompt 质量，不真正执行任务。'
    }
    return '本模式会先把 Prompt 发给执行 AI，再结合执行结果一起评分。'
  }, [strategy])

  const canSubmit = promptDraft.trim().length > 0 && status === 'ready'

  // ── load task ──
  const loadTask = useCallback(async () => {
    if (!session.sessionId) return
    setStatus('loading')
    setErrorMessage('')
    setErrorCode('')
    setPromptDraft('')

    try {
      const result = await loadNextStaticTask(session.sessionId)
      const presentation = result.data

      if (presentation.state === 'completed' || !presentation.task) {
        setErrorMessage('题库状态异常：已完成但仍有题目未提交')
        setErrorCode('TASK_LOAD_MISMATCH')
        setStatus('failed')
        return
      }

      setCurrentTask(presentation.task)
      setProgress(presentation.progress)
      setStatus('ready')
    } catch (err) {
      setErrorMessage(err instanceof Error ? err.message : '题目加载失败')
      setErrorCode('TASK_LOAD_FAILED')
      setStatus('failed')
    }
  }, [session.sessionId])

  // ── initial load ──
  useEffect(() => {
    if (guardError) {
      setErrorMessage(guardError)
      setErrorCode('ROUTE_GUARD_FAILED')
      setStatus('failed')
      return
    }
    void loadTask()
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  // ── handle submit ──
  async function handleSubmit() {
    if (!canSubmit || !currentTask || !session.sessionId) return

    setStatus('submitting')
    setErrorMessage('')
    setErrorCode('')

    try {
      const result = await submitStaticPrompt(
        session.sessionId,
        currentTask.taskId,
        promptDraft,
      )
      const data = result.data

      // Archive current task for result page
      if (data.scoredTask) {
        setTaskReviewItems(prev => [
          ...prev,
          {
            taskId: currentTask.taskId,
            sequenceNo: currentTask.sequenceNo,
            title: currentTask.title,
            task: currentTask.task,
            requirements: currentTask.requirements,
            userPrompt: promptDraft,
            finalScore: data.scoredTask!.finalScore,
            executorOutputPreview: data.scoredTask!.executorOutputPreview,
          },
        ])
      }

      if (data.nextAction === 'show_result') {
        setFinalReport(data.finalReport!)
        setStatus('completed')
      } else {
        setScoredTask(data.scoredTask)
        setProgress(data.progress!)
        setStatus('scoring')
      }
    } catch (err) {
      setErrorMessage(err instanceof Error ? err.message : '提交失败')
      setErrorCode('SUBMIT_FAILED')
      setStatus('failed')
    }
  }

  // ── handle retry ──
  function handleRetry() {
    if (errorCode === 'SUBMIT_FAILED') {
      setStatus('ready')
    } else {
      void loadTask()
    }
  }

  // ── handle next question ──
  function handleNextQuestion() {
    void loadTask()
  }

  // ── handle abandon ──
  function handleAbandon() {
    if (isAbandoning.current) return
    isAbandoning.current = true
    const confirmed = window.confirm('确定要放弃本次测评吗？')
    if (confirmed) {
      setStatus('abandoned')
      navigate('/')
    } else {
      isAbandoning.current = false
    }
  }

  // ── navigate to result page ──
  useEffect(() => {
    if (status === 'completed' && finalReport) {
      navigate('/result', {
        state: {
          origin: 'static_active_session',
          sessionId: session.sessionId,
          sessionSummary: {
            scope: session.scope,
            interactionMode: session.interactionMode,
            questionSource: session.questionSource,
            staticStrategy: session.staticStrategy,
            dynamicEndStrategy: null,
            fixedRoundLimit: null,
            status: 'completed',
            advancedEvaluationContext: session.advancedEvaluationContext,
            questionCount: session.questionCount,
            effectiveRoundCount: 0,
          },
          finalReport,
          reviewData: {
            kind: 'static',
            staticStrategy: strategy,
            taskCount: taskReviewItems.length,
            taskReviewItems,
          },
        },
      })
    }
  }, [status, finalReport, navigate, session, strategy, taskReviewItems])

  // ── abandon guard ──
  useEffect(() => {
    if (status === 'abandoned') {
      navigate('/')
    }
  }, [status, navigate])

  // ── route guard: full page error ──
  if (guardError) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>入口载荷损坏</h2>
          <p>{guardError}</p>
          <Link className="button button--secondary" to="/">
            返回首页
          </Link>
        </div>
      </div>
    )
  }

  // ── failed: session invalid ──
  if (
    status === 'failed' &&
    (errorCode === 'ROUTE_GUARD_FAILED' ||
      errorCode === 'SESSION_NOT_FOUND' ||
      errorCode === 'SESSION_MODE_MISMATCH' ||
      errorCode === 'ASSESSMENT_NOT_ACTIVE')
  ) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>当前会话不可用</h2>
          <p>{errorMessage}</p>
          <Link className="button button--secondary" to="/">
            返回首页
          </Link>
        </div>
      </div>
    )
  }

  // ── failed: load error ──
  if (
    status === 'failed' &&
    (errorCode === 'TASK_LOAD_FAILED' ||
      errorCode === 'TASK_LOAD_MISMATCH' ||
      errorCode === 'QUESTION_NOT_FOUND' ||
      errorCode === 'DB_QUERY_FAILED' ||
      errorCode === 'TASK_NOT_FOUND')
  ) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>题目加载失败</h2>
          <p>{errorMessage}</p>
          <div className="footer-actions">
            <button className="button button--primary" onClick={handleRetry} type="button">
              重新加载题目
            </button>
            <Link className="button button--ghost" to="/">
              返回首页
            </Link>
          </div>
        </div>
      </div>
    )
  }

  // ── failed: TASK_ALREADY_SUBMITTED ──
  if (status === 'failed' && errorCode === 'TASK_ALREADY_SUBMITTED') {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--warning">
          <h2>本题已提交过</h2>
          <p>{errorMessage}</p>
          <button className="button button--primary" onClick={() => void loadTask()} type="button">
            重新加载题目
          </button>
        </div>
      </div>
    )
  }

  // ── loading state ──
  if (status === 'loading' || status === 'idle') {
    return (
      <div className="page-stack">
        <div className="loading-area">
          <p className="eyebrow">正在加载题目</p>
          <div className="spinner" />
        </div>
      </div>
    )
  }

  // ── submitting state ──
  if (status === 'submitting') {
    return (
      <div className="page-stack">
        <PageSection
          title="静态测评"
          description={summaryText}
          actions={<StatusPill label={scopeLabel} tone="info" />}
        >
          <div className="session-banner">
            <div>
              <p className="session-banner__title">
                第 {progress?.currentIndex ?? '?'} / {progress?.totalCount ?? '?'} 题
              </p>
              <p>{summaryText}</p>
            </div>
            <StatusPill label={strategyLabel} tone="neutral" />
          </div>

          <div className="loading-area">
            <p className="eyebrow">正在提交并评分</p>
            <div className="spinner" />
            <p className="supporting-copy">测评 AI 正在评估你的 Prompt，请稍候</p>
          </div>
        </PageSection>
      </div>
    )
  }

  // ── main ready/scoring/failed layout ──
  return (
    <div className="page-stack">
      <PageSection
        title="静态测评"
        description={summaryText}
        actions={<StatusPill label={scopeLabel} tone="info" />}
      >
        {/* ── top session summary bar ── */}
        <div className="session-banner">
          <div>
            <p className="session-banner__title">
              第 {progress?.currentIndex ?? '?'} / {progress?.totalCount ?? '?'} 题
            </p>
            <p className="meta-row">
              <span>{scopeLabel}</span>
              <span className="meta-separator">·</span>
              <span>{sourceLabel}</span>
              <span className="meta-separator">·</span>
              <span>{strategyLabel}</span>
            </p>
          </div>
          <StatusPill label={strategyLabel} tone="neutral" />
        </div>

        {/* ── progress area ── */}
        {progress && (
          <div className="progress-area">
            <div className="progress-area__text">
              已完成 {progress.completedCount} / {progress.totalCount} 题，剩余{' '}
              {progress.remainingCount} 题
            </div>
            <div className="progress-bar">
              <div
                className="progress-bar__fill"
                style={{
                  width: `${(progress.completedCount / progress.totalCount) * 100}%`,
                }}
              />
            </div>
          </div>
        )}

        {/* ── current task card ── */}
        {currentTask && (
          <article className="task-card">
            <div className="task-card__header">
              <div>
                <p className="eyebrow">题目</p>
                <h3>{currentTask.title}</h3>
              </div>
            </div>
            <p>{currentTask.task}</p>
            <div className="tag-row">
              {currentTask.requirements.map((item) => (
                <span key={item} className="tag">
                  {item}
                </span>
              ))}
            </div>
          </article>
        )}

        {/* ── scoring transition panel ── */}
        {status === 'scoring' && scoredTask && (
          <div className="scoring-panel">
            <div className="scoring-panel__header">
              <StatusPill label="本题已完成" tone="success" />
              <span className="scoring-panel__score">
                得分：<strong>{Math.round(scoredTask.finalScore)}</strong>
              </span>
            </div>
            {scoredTask.executorOutputPreview && (
              <details className="scoring-panel__details">
                <summary>查看本题执行结果摘要</summary>
                <pre className="scoring-panel__preview">{scoredTask.executorOutputPreview}</pre>
              </details>
            )}
            <div className="footer-actions">
              <button
                className="button button--primary"
                onClick={handleNextQuestion}
                type="button"
              >
                下一题
              </button>
            </div>
          </div>
        )}

        {/* ── failed error card (submit failure) ── */}
        {status === 'failed' && errorCode === 'SUBMIT_FAILED' && (
          <div className="stacked-alert stacked-alert--danger">
            <h2>提交失败</h2>
            <p>{errorMessage}</p>
            <p className="supporting-copy">请检查配置后重试，或编辑输入后再次提交</p>
            <div className="footer-actions">
              <button className="button button--primary" onClick={handleRetry} type="button">
                重试提交
              </button>
              <button className="button button--ghost" onClick={handleAbandon} type="button">
                放弃本次测评
              </button>
            </div>
          </div>
        )}

        {/* ── failed error card (load failure on next) ── */}
        {status === 'failed' &&
          errorCode !== 'SUBMIT_FAILED' &&
          errorCode !== 'ROUTE_GUARD_FAILED' &&
          errorCode !== 'SESSION_NOT_FOUND' &&
          errorCode !== 'SESSION_MODE_MISMATCH' &&
          errorCode !== 'ASSESSMENT_NOT_ACTIVE' &&
          errorCode !== 'TASK_LOAD_FAILED' &&
          errorCode !== 'TASK_LOAD_MISMATCH' &&
          errorCode !== 'QUESTION_NOT_FOUND' &&
          errorCode !== 'DB_QUERY_FAILED' &&
          errorCode !== 'TASK_NOT_FOUND' &&
          errorCode !== 'TASK_ALREADY_SUBMITTED' && (
            <div className="stacked-alert stacked-alert--danger">
              <h2>操作失败</h2>
              <p>{errorMessage}</p>
              <div className="footer-actions">
                <button className="button button--primary" onClick={handleRetry} type="button">
                  重试
                </button>
                <button className="button button--ghost" onClick={handleAbandon} type="button">
                  放弃本次测评
                </button>
              </div>
            </div>
          )}
      </PageSection>

      {/* ── prompt input area ── */}
      {status !== 'scoring' && status !== 'submitting' && (
        <PageSection title="你的 Prompt" description="把任务背景、约束和输出格式写清楚。">
          <label className="field field--full">
            <span>Prompt 内容</span>
            <textarea
              rows={12}
              value={promptDraft}
              onChange={(event) => setPromptDraft(event.target.value)}
              placeholder="请把任务背景、约束、输出格式和缺失条件补问策略写清楚。"
              disabled={status === 'submitting'}
            />
          </label>

          <div className="footer-actions">
            <button
              className="button button--primary"
              disabled={!canSubmit}
              onClick={handleSubmit}
              type="button"
            >
              提交本题
            </button>
            <button
              className="button button--secondary"
              disabled={status === 'submitting'}
              onClick={() => setPromptDraft('')}
              type="button"
            >
              清空输入
            </button>
            <button
              className="button button--ghost"
              disabled={status === 'submitting'}
              onClick={handleAbandon}
              type="button"
            >
              放弃本次测评
            </button>
          </div>
        </PageSection>
      )}
    </div>
  )
}
