import { useMemo, useState } from 'react'
import { Link, useLocation, useNavigate } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import {
  mockCoreScores,
  mockDynamicMessages,
  mockDynamicSnapshot,
  mockResultReport,
  type ChatMessage,
  type DynamicEndStrategy,
} from '../data/mock'

type DynamicRouteState = {
  sessionId?: string
  scope?: 'fun' | 'full'
  questionSource?: 'builtin' | 'ai_generated'
  dynamicEndStrategy?: DynamicEndStrategy
  fixedRoundLimit?: number | null
}

function createMessage(
  role: ChatMessage['role'],
  content: string,
): ChatMessage {
  return {
    messageId: `${role}-${Date.now()}-${Math.random().toString(16).slice(2, 8)}`,
    role,
    content,
    createdAt: new Date().toLocaleString('zh-CN', { hour12: false }),
    deliveryState: role === 'user' ? 'sent' : undefined,
  }
}

export function DynamicAssessmentPage() {
  const navigate = useNavigate()
  const location = useLocation()
  const routeState = (location.state as DynamicRouteState | null) ?? {}
  const [draft, setDraft] = useState('')
  const [messages, setMessages] = useState<ChatMessage[]>(mockDynamicMessages)

  const scopeLabel = routeState.scope === 'full' ? '全面测评' : '趣味速测'
  const endStrategy = routeState.dynamicEndStrategy ?? 'fixed_rounds'
  const roundLimit = routeState.fixedRoundLimit ?? 6
  const completedRounds = useMemo(() => {
    return Math.max(
      0,
      messages.filter((message) => message.role === 'executor').length - 1,
    )
  }, [messages])

  function handleSend() {
    const trimmed = draft.trim()
    if (!trimmed) {
      return
    }

    const userMessage = createMessage('user', trimmed)
    const executorMessage = createMessage(
      'executor',
      `收到。基于你刚才的要求，我会继续把任务拆得更清楚，并补上输出结构与异常约束。当前重点是：${trimmed.slice(0, 24)}${trimmed.length > 24 ? '...' : ''}`,
    )

    setMessages((current) => [...current, userMessage, executorMessage])
    setDraft('')
  }

  function handleFinish() {
    navigate('/result', {
      state: {
        origin: '/assess/dynamic',
        report: {
          ...mockResultReport,
          sessionId: routeState.sessionId ?? 'mock-dynamic-session',
          interactionMode: 'dynamic',
          title: '动态测评结果',
          totalScore: mockDynamicSnapshot.overallScore,
          summary: '用户能够和执行 AI 展开协作，但对缺失条件的追问仍然偏少。',
        },
      },
    })
  }

  return (
    <div className="page-stack">
      <PageSection
        title="动态测评"
        description="执行 AI 负责对话，测评 AI 只在侧边栏持续估分与观察。"
        actions={<StatusPill label={scopeLabel} tone="info" />}
      >
        <div className="session-banner session-banner--split">
          <div>
            <p className="session-banner__title">当前任务</p>
            <p>围绕一个设置页规格整理任务，与执行 AI 多轮协作，直到任务足够清晰。</p>
          </div>
          <div className="session-banner__meta">
            <StatusPill
              label={
                endStrategy === 'fixed_rounds'
                  ? `固定 ${roundLimit} 轮`
                  : '用户主动结束'
              }
              tone="neutral"
            />
            <StatusPill label={`当前 ${completedRounds} 轮`} tone="neutral" />
          </div>
        </div>
      </PageSection>

      <div className="content-grid content-grid--chat">
        <PageSection
          title="协作聊天"
          description="首版骨架只使用本地 mock 对话，不接真实模型调用。"
          className="content-grid__main"
        >
          <div className="chat-thread">
            {messages.map((message) => (
              <article
                key={message.messageId}
                className={`chat-bubble chat-bubble--${message.role}`}
              >
                <div className="chat-bubble__meta">
                  <strong>
                    {message.role === 'user'
                      ? '你'
                      : message.role === 'executor'
                        ? '执行 AI'
                        : '系统任务'}
                  </strong>
                  <span>{message.createdAt}</span>
                </div>
                <p>{message.content}</p>
              </article>
            ))}
          </div>

          <label className="field field--full">
            <span>继续发送给执行 AI</span>
            <textarea
              rows={5}
              value={draft}
              onChange={(event) => setDraft(event.target.value)}
              placeholder="继续细化背景、约束、输出格式，或主动补问缺失条件。"
            />
          </label>

          <div className="footer-actions">
            <button className="button button--primary" onClick={handleSend}>
              发送
            </button>
            <button className="button button--secondary" onClick={handleFinish}>
              完成测评
            </button>
            <Link className="button button--ghost" to="/assess/setup">
              返回配置页
            </Link>
          </div>
        </PageSection>

        <PageSection
          title="实时估分"
          description="动态过程中只显示趋势与当前估分，最终以结束结算为准。"
          className="content-grid__side"
        >
          <div className="score-summary">
            <div>
              <p className="eyebrow">当前估分</p>
              <strong className="score-number">
                {mockDynamicSnapshot.overallScore}
              </strong>
            </div>
            <StatusPill
              label={
                mockDynamicSnapshot.scoreTrend === 'up'
                  ? '趋势上升'
                  : mockDynamicSnapshot.scoreTrend === 'down'
                    ? '趋势下降'
                    : '趋势持平'
              }
              tone="success"
            />
          </div>
          <p className="supporting-copy">
            {mockDynamicSnapshot.shortObservation}
          </p>
          <div className="score-list">
            {mockCoreScores.map((item) => (
              <article key={item.key} className="score-list__item">
                <div className="score-list__head">
                  <h3>{item.label}</h3>
                  <span>{item.score}</span>
                </div>
                <p>{item.emphasis}</p>
              </article>
            ))}
          </div>
        </PageSection>
      </div>
    </div>
  )
}
