import { Link, useLocation } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import { mockResultReport, type FinalReportSummary } from '../data/mock'

type ResultRouteState = {
  origin?: string
  report?: FinalReportSummary
  submittedPrompt?: string
}

export function ResultReportPage() {
  const location = useLocation()
  const routeState = (location.state as ResultRouteState | null) ?? {}
  const report = routeState.report

  if (!report) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>结果数据不可用</h2>
          <p>当前页面没有收到上游结果载荷，无法直接恢复本次测评结果。</p>
          <div className="footer-actions">
            <Link className="button button--primary" to="/">
              返回首页
            </Link>
            <Link className="button button--secondary" to="/history">
              查看历史记录
            </Link>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="page-stack">
      <PageSection
        title={report.title}
        description="首版结果页先接固定章节和 mock 数据，后续再接 Markdown 导出与真实证据引用。"
        actions={
          <StatusPill
            label={report.interactionMode === 'static' ? '静态测评' : '动态测评'}
            tone="info"
          />
        }
      >
        <div className="result-hero">
          <div>
            <p className="eyebrow">总分</p>
            <strong className="score-number score-number--xl">
              {report.totalScore}
            </strong>
          </div>
          <p className="supporting-copy">{report.summary}</p>
        </div>
      </PageSection>

      {routeState.submittedPrompt ? (
        <PageSection
          title="本题回顾"
          description="静态测评场景下保留本次提交的 Prompt 草稿。"
        >
          <pre className="code-panel">{routeState.submittedPrompt}</pre>
        </PageSection>
      ) : null}

      <PageSection
        title="核心维度"
        description="首版结果页按雷达图对应的核心维度列表展示。"
      >
        <div className="score-list">
          {report.coreScores.map((item) => (
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

      <div className="card-grid card-grid--two">
        <PageSection title="表现较好" description="本次测评中最稳定的部分。">
          <ul className="plain-list">
            {report.strengths.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
        </PageSection>

        <PageSection title="主要问题" description="优先修正这些点，分数会更容易提升。">
          <ul className="plain-list">
            {report.issues.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
        </PageSection>
      </div>

      <PageSection title="改进建议" description="结束后才给出建议，不在动态过程里介入主对话。">
        <ul className="plain-list">
          {report.suggestions.map((item) => (
            <li key={item}>{item}</li>
          ))}
        </ul>
      </PageSection>

      <PageSection title="证据摘录" description="首版用短摘录代替完整证据定位。">
        <ul className="plain-list">
          {report.evidenceHighlights.map((item) => (
            <li key={item}>{item}</li>
          ))}
        </ul>
      </PageSection>

      <PageSection title="后续操作" description="导出按钮暂时用占位形式保留页面结构。">
        <div className="footer-actions">
          <button className="button button--primary" type="button">
            导出 Markdown 报告
          </button>
          <Link className="button button--secondary" to="/assess/setup">
            重新测试
          </Link>
          <Link className="button button--ghost" to={routeState.origin ?? '/'}>
            返回上一页
          </Link>
        </div>
      </PageSection>
    </div>
  )
}

export function buildHistoryReport(sessionId: string): FinalReportSummary {
  return {
    ...mockResultReport,
    sessionId,
    title: `历史详情 · ${sessionId}`,
  }
}
