import { Link, useParams } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import { mockHistory } from '../data/mock'
import { buildHistoryReport } from './ResultReportPage'

export function HistoryDetailPage() {
  const params = useParams()
  const sessionId = params.sessionId ?? ''
  const historyItem = mockHistory.find((item) => item.sessionId === sessionId)

  if (!historyItem) {
    return (
      <div className="page-stack">
        <div className="stacked-alert stacked-alert--danger">
          <h2>历史记录不存在</h2>
          <p>当前 sessionId 没有匹配到本地 mock 历史记录。</p>
          <Link className="button button--primary" to="/history">
            返回历史总览
          </Link>
        </div>
      </div>
    )
  }

  const report = buildHistoryReport(historyItem.sessionId)

  return (
    <div className="page-stack">
      <PageSection
        title="历史详情"
        description="这里保留总览中不展示的完整结果内容、操作按钮和删除入口。"
        actions={<StatusPill label={`${historyItem.totalScore} 分`} tone="info" />}
      >
        <div className="table-list table-list--compact">
          <article className="table-list__row">
            <div>
              <strong>测评模型</strong>
            </div>
            <span>{historyItem.evaluatorModel}</span>
          </article>
          <article className="table-list__row">
            <div>
              <strong>测评时间</strong>
            </div>
            <span>{historyItem.evaluatedAt}</span>
          </article>
          <article className="table-list__row">
            <div>
              <strong>测评模式</strong>
            </div>
            <span>{historyItem.interactionMode === 'static' ? '静态' : '动态'}</span>
          </article>
        </div>
      </PageSection>

      <PageSection title="结果摘要" description={report.summary}>
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

      <PageSection title="详情操作" description="导出与删除先保留 UI 入口，不接真实后端写入。">
        <div className="footer-actions">
          <button className="button button--primary" type="button">
            导出 Markdown 报告
          </button>
          <button className="button button--secondary" type="button">
            删除此记录
          </button>
          <Link className="button button--ghost" to="/history">
            返回历史总览
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
