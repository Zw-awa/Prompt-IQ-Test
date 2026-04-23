import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import { mockHistory } from '../data/mock'

export function HistoryListPage() {
  return (
    <div className="page-stack">
      <PageSection
        title="历史记录"
        description="首版总览页只展示测评模型、测评时间和总分，点详情再展开完整内容。"
      >
        <div className="table-list">
          <div className="table-list__head">
            <span>测评模型</span>
            <span>测评时间</span>
            <span>测评总分</span>
            <span>操作</span>
          </div>
          {mockHistory.map((item) => (
            <article key={item.sessionId} className="table-list__row">
              <div>
                <strong>{item.evaluatorModel}</strong>
                <p>{item.title}</p>
              </div>
              <span>{item.evaluatedAt}</span>
              <StatusPill label={`${item.totalScore}`} tone="info" />
              <div className="inline-actions">
                <Link
                  className="text-link"
                  to={`/history/${item.sessionId}`}
                  state={{ sessionId: item.sessionId }}
                >
                  详情 / 展开
                </Link>
              </div>
            </article>
          ))}
        </div>

        <div className="footer-actions">
          <button className="button button--secondary" type="button">
            删除全部记录
          </button>
          <Link className="button button--ghost" to="/">
            返回首页
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
