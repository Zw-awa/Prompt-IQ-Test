import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'

export function VentGamePage() {
  return (
    <div className="page-stack">
      <PageSection
        title="发泄小游戏"
        description="当前版本只保留占位结构，不开放真实聊天入口。"
      >
        <div className="stacked-alert stacked-alert--warning">
          <h3>发泄小游戏正在开发中</h3>
          <p>当前版本尚未开放。</p>
        </div>
      </PageSection>

      <PageSection title="未来能力" description="首版只说明方向，不执行上传动作。">
        <ul className="plain-list">
          <li>一个专门吐槽和发泄 AI 的窗口</li>
          <li>AI 只会夸张认错和接话</li>
          <li>本地模式</li>
          <li>可选上传开关</li>
        </ul>
        <div className="footer-actions">
          <Link className="button button--ghost" to="/">
            返回首页
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
