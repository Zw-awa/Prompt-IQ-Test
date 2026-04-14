import { useMemo, useState } from 'react'
import { Link, useLocation, useNavigate } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import { mockResultReport, mockStaticTask, type StaticStrategy } from '../data/mock'

type StaticRouteState = {
  sessionId?: string
  scope?: 'fun' | 'full'
  questionSource?: 'builtin' | 'ai_generated'
  staticStrategy?: StaticStrategy
  questionCount?: number
  initialTaskReady?: boolean
}

export function StaticAssessmentPage() {
  const navigate = useNavigate()
  const location = useLocation()
  const routeState = (location.state as StaticRouteState | null) ?? {}
  const [promptDraft, setPromptDraft] = useState('')
  const [submitted, setSubmitted] = useState(false)

  const strategy = routeState.staticStrategy ?? 'prompt_only'
  const questionCount = routeState.questionCount ?? 1
  const ready = routeState.initialTaskReady ?? true

  const canSubmit = promptDraft.trim().length > 0 && !submitted
  const scopeLabel = routeState.scope === 'full' ? '全面测评' : '趣味速测'

  const summaryText = useMemo(() => {
    if (strategy === 'prompt_only') {
      return '本模式只评估你写出的 Prompt 质量，不真正执行任务。'
    }

    return '本模式会先把 Prompt 发给执行 AI，再结合执行结果一起评分。'
  }, [strategy])

  function handleSubmit() {
    if (!canSubmit) {
      return
    }

    setSubmitted(true)
    navigate('/result', {
      state: {
        origin: '/assess/static',
        report: {
          ...mockResultReport,
          sessionId: routeState.sessionId ?? mockResultReport.sessionId,
          title:
            strategy === 'prompt_only'
              ? '静态测评结果 · 只评 Prompt'
              : '静态测评结果 · 执行后再评',
        },
        submittedPrompt: promptDraft,
      },
    })
  }

  if (!ready) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>当前题面不可用</h2>
          <p>入口载荷缺失，静态测评页暂时无法恢复。</p>
          <Link className="button button--secondary" to="/assess/setup">
            返回配置页
          </Link>
        </div>
      </div>
    )
  }

  return (
    <div className="page-stack">
      <PageSection
        title="静态测评"
        description="首版一次只展示 1 题，做完当前题再进入下一题。"
        actions={<StatusPill label={scopeLabel} tone="info" />}
      >
        <div className="session-banner">
          <div>
            <p className="session-banner__title">
              第 {mockStaticTask.progress.currentIndex} / {questionCount} 题
            </p>
            <p>{summaryText}</p>
          </div>
          <StatusPill
            label={strategy === 'prompt_only' ? '只评 Prompt' : '执行后再评'}
            tone="neutral"
          />
        </div>

        <article className="task-card">
          <div className="task-card__header">
            <div>
              <p className="eyebrow">题目</p>
              <h3>{mockStaticTask.title}</h3>
            </div>
            <StatusPill
              label={`${mockStaticTask.progress.completedCount} 已完成`}
              tone="neutral"
            />
          </div>
          <p>{mockStaticTask.task}</p>
          <div className="tag-row">
            {mockStaticTask.requirements.map((item) => (
              <span key={item} className="tag">
                {item}
              </span>
            ))}
          </div>
        </article>
      </PageSection>

      <PageSection
        title="你的 Prompt"
        description="这里先放前端草稿输入框，后续再接入实际提交、锁定、评分与多题推进逻辑。"
      >
        <label className="field field--full">
          <span>Prompt 内容</span>
          <textarea
            rows={12}
            value={promptDraft}
            onChange={(event) => setPromptDraft(event.target.value)}
            placeholder="请把任务背景、约束、输出格式和缺失条件补问策略写清楚。"
          />
        </label>

        <div className="footer-actions">
          <button className="button button--primary" disabled={!canSubmit} onClick={handleSubmit}>
            提交本题
          </button>
          <button
            className="button button--secondary"
            disabled={submitted}
            onClick={() => setPromptDraft('')}
          >
            清空输入
          </button>
          <Link className="button button--ghost" to="/assess/setup">
            放弃本次测评
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
