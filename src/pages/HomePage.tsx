import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import type { AppBootstrap, SavedProfilesSummary } from '../data/mock'

type HomePageProps = {
  bootstrap: AppBootstrap
  profiles: SavedProfilesSummary
  runtimeLabel: string
}

export function HomePage({
  bootstrap,
  profiles,
  runtimeLabel,
}: HomePageProps) {
  return (
    <div className="page-stack">
      <section className="hero-panel">
        <div className="hero-panel__badge-row">
          <StatusPill label="首版工程骨架" tone="info" />
          <StatusPill label={runtimeLabel} tone="neutral" />
        </div>
        <div className="hero-panel__content">
          <div>
            <p className="eyebrow">Prompt IQ Test</p>
            <h1>判断你会不会用 AI，也判断你能不能高质量驱动 Agent。</h1>
            <p className="hero-panel__summary">
              首版聚焦普通用户与进阶用户两类判断：一类看是否会用 AI，
              一类看是否能把 LLM/Agent 的能力真正发挥出来。
            </p>
          </div>
          <div className="hero-panel__actions">
            <Link className="button button--primary" to="/assess/setup">
              开始测评
            </Link>
            <Link className="button button--secondary" to="/history">
              查看历史
            </Link>
          </div>
        </div>
      </section>

      <PageSection
        title="当前配置状态"
        description="首页只显示安全摘要，不显示 API Key 明文。"
      >
        <div className="card-grid card-grid--three">
          <article className="info-card">
            <div className="info-card__top">
              <h3>执行 AI</h3>
              <StatusPill
                label={
                  bootstrap.profileAvailability.executorConfigured
                    ? '已配置'
                    : '未配置'
                }
                tone={
                  bootstrap.profileAvailability.executorConfigured
                    ? 'success'
                    : 'warning'
                }
              />
            </div>
            <p>{profiles.executor?.model ?? '未填写模型'}</p>
          </article>

          <article className="info-card">
            <div className="info-card__top">
              <h3>测评 AI</h3>
              <StatusPill
                label={
                  bootstrap.profileAvailability.evaluatorConfigured
                    ? '已配置'
                    : '未配置'
                }
                tone={
                  bootstrap.profileAvailability.evaluatorConfigured
                    ? 'success'
                    : 'warning'
                }
              />
            </div>
            <p>{profiles.evaluator?.model ?? '未填写模型'}</p>
          </article>

          <article className="info-card">
            <div className="info-card__top">
              <h3>出题 AI</h3>
              <StatusPill
                label={
                  profiles.generator.source === 'evaluator'
                    ? '复用测评模型'
                    : '独立配置'
                }
                tone="neutral"
              />
            </div>
            <p>
              {profiles.generator.profile?.model ??
                '当前默认复用测评 AI 作为出题 AI'}
            </p>
          </article>
        </div>
      </PageSection>

      <PageSection
        title="测评入口"
        description="首版同时保留趣味速测 / 全面测评、静态 / 动态、题库 / AI 出题三组选择。"
      >
        <div className="card-grid card-grid--three">
          <article className="feature-card">
            <h3>趣味速测</h3>
            <p>偏体验型，题量更少，适合快速感知自己会不会用 AI。</p>
          </article>
          <article className="feature-card">
            <h3>全面测评</h3>
            <p>多角度、多轮次覆盖，尽量减少误判，更接近真实能力画像。</p>
          </article>
          <article className="feature-card">
            <h3>动态协作</h3>
            <p>执行 AI 负责对话，测评 AI 只在侧边栏持续评分与观察，不介入主对话。</p>
          </article>
        </div>
      </PageSection>

      <PageSection
        title="其他入口"
        description="即使没有配置 API，这些页面也都可以独立进入。"
      >
        <div className="inline-actions inline-actions--wrap">
          <Link className="button button--secondary" to="/settings">
            设置
          </Link>
          <Link className="button button--secondary" to="/about">
            关于
          </Link>
          <Link className="button button--secondary" to="/plugins">
            插件商场
          </Link>
          <Link className="button button--secondary" to="/vent">
            发泄小游戏
          </Link>
        </div>
      </PageSection>

      <PageSection
        title="本地优先说明"
        description="程序本身不上传你的配置和记录，但第三方模型服务商仍可能处理你发送给它的请求。"
      >
        <div className="notice-card">
          <p>
            应用版本 {bootstrap.appMeta.version}，许可证 {bootstrap.appMeta.license}
            ，历史记录 {bootstrap.historySummary.completedCount} 条，最近完成时间{' '}
            {bootstrap.historySummary.latestCompletedAt ?? '暂无'}。
          </p>
          <div className="inline-actions">
            <Link className="text-link" to="/about">
              查看关于与免责声明
            </Link>
            <Link className="text-link" to="/settings">
              打开设置页
            </Link>
          </div>
        </div>
      </PageSection>
    </div>
  )
}
