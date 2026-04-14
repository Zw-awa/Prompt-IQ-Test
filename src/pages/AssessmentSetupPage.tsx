import { useMemo, useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import type {
  AppBootstrap,
  DynamicEndStrategy,
  InteractionMode,
  QuestionSource,
  SavedProfilesSummary,
  ScopeMode,
  StaticStrategy,
} from '../data/mock'

type AssessmentSetupPageProps = {
  bootstrap: AppBootstrap
  profiles: SavedProfilesSummary
}

export function AssessmentSetupPage({
  bootstrap,
  profiles,
}: AssessmentSetupPageProps) {
  const navigate = useNavigate()
  const [scope, setScope] = useState<ScopeMode>('fun')
  const [interactionMode, setInteractionMode] =
    useState<InteractionMode>('static')
  const [questionSource, setQuestionSource] =
    useState<QuestionSource>('builtin')
  const [staticStrategy, setStaticStrategy] =
    useState<StaticStrategy>('prompt_only')
  const [dynamicEndStrategy, setDynamicEndStrategy] =
    useState<DynamicEndStrategy>('fixed_rounds')
  const [fixedRoundLimit, setFixedRoundLimit] = useState('6')
  const [generationConstraints, setGenerationConstraints] = useState('')
  const [advancedEvaluationContext, setAdvancedEvaluationContext] = useState(
    bootstrap.settingsSummary.defaultAdvancedEvaluationContext,
  )
  const [executorModelOverride, setExecutorModelOverride] = useState('')
  const [evaluatorModelOverride, setEvaluatorModelOverride] = useState('')
  const [generatorModelOverride, setGeneratorModelOverride] = useState('')

  const requiresExecutor =
    interactionMode === 'dynamic' || staticStrategy === 'execute_then_evaluate'
  const requiresEvaluator = true
  const requiresGenerator = questionSource === 'ai_generated'

  const blockReasons = useMemo(() => {
    const reasons: string[] = []

    if (requiresEvaluator && !profiles.evaluator?.hasApiKey) {
      reasons.push('测评 AI 尚未配置，无法启动任何测评。')
    }

    if (requiresExecutor && !profiles.executor?.hasApiKey) {
      reasons.push('当前模式依赖执行 AI，但执行模型尚未配置。')
    }

    if (
      requiresGenerator &&
      profiles.generator.source === 'dedicated' &&
      !profiles.generator.profile?.hasApiKey
    ) {
      reasons.push('当前为独立出题模型，但出题 AI 尚未配置。')
    }

    if (
      interactionMode === 'dynamic' &&
      dynamicEndStrategy === 'fixed_rounds' &&
      (!fixedRoundLimit || Number(fixedRoundLimit) < 1)
    ) {
      reasons.push('固定轮数必须是大于等于 1 的整数。')
    }

    return reasons
  }, [
    dynamicEndStrategy,
    fixedRoundLimit,
    interactionMode,
    profiles.evaluator,
    profiles.executor,
    profiles.generator,
    requiresEvaluator,
    requiresExecutor,
    requiresGenerator,
  ])

  const isBlocked = blockReasons.length > 0

  function handleStart() {
    if (isBlocked) {
      return
    }

    if (interactionMode === 'static') {
      navigate('/assess/static', {
        state: {
          sessionId: 'mock-static-session',
          scope,
          questionSource,
          staticStrategy,
          questionCount: scope === 'fun' ? 1 : 5,
          initialTaskReady: true,
        },
      })
      return
    }

    navigate('/assess/dynamic', {
      state: {
        sessionId: 'mock-dynamic-session',
        scope,
        questionSource,
        dynamicEndStrategy,
        fixedRoundLimit:
          dynamicEndStrategy === 'fixed_rounds' ? Number(fixedRoundLimit) : null,
      },
    })
  }

  return (
    <div className="page-stack">
      <PageSection
        title="开始测评"
        description="首版保持单页配置表单，用户在进入测评前一次性选完模式组合。"
        actions={
          <Link className="button button--ghost" to="/">
            返回首页
          </Link>
        }
      >
        <div className="form-grid form-grid--two">
          <label className="field">
            <span>测评范围</span>
            <select
              value={scope}
              onChange={(event) => setScope(event.target.value as ScopeMode)}
            >
              <option value="fun">趣味速测</option>
              <option value="full">全面测评</option>
            </select>
          </label>

          <label className="field">
            <span>交互模式</span>
            <select
              value={interactionMode}
              onChange={(event) =>
                setInteractionMode(event.target.value as InteractionMode)
              }
            >
              <option value="static">静态测评</option>
              <option value="dynamic">动态测评</option>
            </select>
          </label>

          <label className="field">
            <span>题目来源</span>
            <select
              value={questionSource}
              onChange={(event) =>
                setQuestionSource(event.target.value as QuestionSource)
              }
            >
              <option value="builtin">题库抽样</option>
              <option value="ai_generated">AI 实时出题</option>
            </select>
          </label>

          {interactionMode === 'static' ? (
            <label className="field">
              <span>静态评分方式</span>
              <select
                value={staticStrategy}
                onChange={(event) =>
                  setStaticStrategy(event.target.value as StaticStrategy)
                }
              >
                <option value="prompt_only">只评 Prompt</option>
                <option value="execute_then_evaluate">执行后再评</option>
              </select>
            </label>
          ) : (
            <label className="field">
              <span>动态结束策略</span>
              <select
                value={dynamicEndStrategy}
                onChange={(event) =>
                  setDynamicEndStrategy(
                    event.target.value as DynamicEndStrategy,
                  )
                }
              >
                <option value="fixed_rounds">固定轮数自动结束</option>
                <option value="manual_with_evaluator_suggestion">
                  用户主动结束 + 测评 AI 建议
                </option>
              </select>
            </label>
          )}

          {interactionMode === 'dynamic' &&
          dynamicEndStrategy === 'fixed_rounds' ? (
            <label className="field">
              <span>固定轮数</span>
              <input
                type="number"
                min="1"
                value={fixedRoundLimit}
                onChange={(event) => setFixedRoundLimit(event.target.value)}
              />
            </label>
          ) : null}

          {questionSource === 'ai_generated' ? (
            <label className="field field--full">
              <span>出题限定词</span>
              <textarea
                rows={4}
                value={generationConstraints}
                onChange={(event) =>
                  setGenerationConstraints(event.target.value)
                }
                placeholder="可留空；也可以给 AI 一些限定词，让它围绕这些要求出题。"
              />
            </label>
          ) : null}
        </div>
      </PageSection>

      <PageSection
        title="运行时模型覆盖"
        description="首版只允许在测评配置页临时覆盖 model，不允许改 API Key 或 baseUrl。"
      >
        <div className="form-grid form-grid--three">
          <label className="field">
            <span>执行 AI 临时模型</span>
            <input
              value={executorModelOverride}
              onChange={(event) => setExecutorModelOverride(event.target.value)}
              placeholder={profiles.executor?.model ?? '未配置'}
            />
          </label>
          <label className="field">
            <span>测评 AI 临时模型</span>
            <input
              value={evaluatorModelOverride}
              onChange={(event) => setEvaluatorModelOverride(event.target.value)}
              placeholder={profiles.evaluator?.model ?? '未配置'}
            />
          </label>
          <label className="field">
            <span>出题 AI 临时模型</span>
            <input
              value={generatorModelOverride}
              onChange={(event) => setGeneratorModelOverride(event.target.value)}
              placeholder={
                profiles.generator.profile?.model ??
                profiles.evaluator?.model ??
                '复用测评模型'
              }
            />
          </label>
        </div>
      </PageSection>

      <PageSection
        title="高级开关"
        description="高级模式会向测评 AI 注入更多上下文，并允许在后续版本扩展隐藏提示词可见性。"
      >
        <div className="toggle-row">
          <label className="checkbox-field">
            <input
              type="checkbox"
              checked={advancedEvaluationContext}
              onChange={(event) =>
                setAdvancedEvaluationContext(event.target.checked)
              }
            />
            <span>启用高级评分上下文</span>
          </label>
          <StatusPill
            label={advancedEvaluationContext ? '已启用' : '默认基础模式'}
            tone={advancedEvaluationContext ? 'info' : 'neutral'}
          />
        </div>
      </PageSection>

      <PageSection
        title="启动条件"
        description="主按钮会根据当前模式组合和配置摘要判断是否允许开始。"
      >
        {isBlocked ? (
          <div className="stacked-alert stacked-alert--warning">
            <h3>当前无法开始测评</h3>
            <ul className="plain-list">
              {blockReasons.map((reason) => (
                <li key={reason}>{reason}</li>
              ))}
            </ul>
            <Link className="button button--secondary" to="/settings">
              前往设置页补配置
            </Link>
          </div>
        ) : (
          <div className="stacked-alert stacked-alert--success">
            <h3>当前配置可启动</h3>
            <p>
              本次将以
              {scope === 'fun' ? '趣味速测' : '全面测评'}
              、{interactionMode === 'static' ? '静态测评' : '动态测评'}、
              {questionSource === 'builtin' ? '题库抽样' : 'AI 出题'} 进入流程。
            </p>
          </div>
        )}

        <div className="footer-actions">
          <button
            className="button button--primary"
            disabled={isBlocked}
            onClick={handleStart}
          >
            开始测评
          </button>
          <Link className="button button--ghost" to="/">
            取消并返回
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
