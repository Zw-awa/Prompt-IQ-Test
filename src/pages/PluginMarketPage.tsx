import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'

export function PluginMarketPage() {
  return (
    <div className="page-stack">
      <PageSection
        title="插件商场"
        description="当前版本只是占位页，不展示可点击的假插件列表。"
      >
        <div className="stacked-alert stacked-alert--warning">
          <h3>插件商场正在开发中</h3>
          <p>当前版本尚未开放插件安装与运行能力。</p>
        </div>
      </PageSection>

      <PageSection title="未来方向" description="这里先冻结未来扩展方向，不给出无法保证的上线承诺。">
        <ul className="plain-list">
          <li>评分维度插件</li>
          <li>题库插件</li>
          <li>模型适配插件</li>
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
