import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'

export function AboutPage() {
  return (
    <div className="page-stack">
      <PageSection
        title="关于"
        description="Prompt IQ Test 的作者、项目和数据边界说明。"
      >
        <div className="card-grid card-grid--two">
          <article className="info-card">
            <h3>作者信息</h3>
            <p>待作者填写</p>
            <p>嵌入式 + AI 方向学生，正在把产品想法沉淀成可落地的本地工具。</p>
          </article>

          <article className="info-card">
            <h3>项目信息</h3>
            <p>Prompt IQ Test</p>
            <p>一个本地优先的 AI 使用能力测评工具。</p>
            <p>License: Apache-2.0</p>
          </article>
        </div>
      </PageSection>

      <PageSection title="项目链接" description="首版只在系统浏览器中打开外部链接。">
        <div className="inline-actions">
          <a className="text-link" href="https://github.com/<your-profile>" target="_blank" rel="noreferrer">
            GitHub 主页
          </a>
          <a className="text-link" href="https://github.com/<your-repo>" target="_blank" rel="noreferrer">
            项目仓库
          </a>
        </div>
      </PageSection>

      <PageSection title="隐私与开源说明" description="这些内容必须在关于页明确呈现。">
        <ul className="plain-list">
          <li>程序不会向作者服务器上传信息。</li>
          <li>程序完全免费开源。</li>
          <li>实际费用来自你调用第三方模型服务的 Token 消耗。</li>
          <li>项目采用 Apache-2.0。</li>
        </ul>
      </PageSection>

      <PageSection title="免责声明" description="首版固定展示，不允许弱化。">
        <ul className="plain-list">
          <li>作者不对程序使用后果负责。</li>
          <li>第三方 AI 服务商仍可能接触你主动发送的请求内容。</li>
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
