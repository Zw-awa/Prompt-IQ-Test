import { useEffect, useMemo, useState } from 'react'
import { HashRouter, NavLink, Route, Routes } from 'react-router-dom'

import { HomePage } from './pages/HomePage'
import { AssessmentSetupPage } from './pages/AssessmentSetupPage'
import { StaticAssessmentPage } from './pages/StaticAssessmentPage'
import { DynamicAssessmentPage } from './pages/DynamicAssessmentPage'
import { ResultReportPage } from './pages/ResultReportPage'
import { HistoryListPage } from './pages/HistoryListPage'
import { HistoryDetailPage } from './pages/HistoryDetailPage'
import { SettingsPage } from './pages/SettingsPage'
import { AboutPage } from './pages/AboutPage'
import { PluginMarketPage } from './pages/PluginMarketPage'
import { VentGamePage } from './pages/VentGamePage'
import { NotFoundPage } from './pages/NotFoundPage'
import {
  mockBootstrap,
  mockProfiles,
  type AppBootstrap,
  type SavedProfilesSummary,
} from './data/mock'
import {
  getAppBootstrap,
  getSavedProfilesSummary,
  isTauriRuntime,
  type CommandWarning,
  type SettingsSummary,
} from './lib/tauri'

type BootStatus = 'loading' | 'ready' | 'failed'

const navItems = [
  { to: '/', label: '首页', end: true },
  { to: '/assess/setup', label: '开始测评' },
  { to: '/history', label: '历史记录' },
  { to: '/settings', label: '设置' },
  { to: '/about', label: '关于' },
  { to: '/plugins', label: '插件商场' },
  { to: '/vent', label: '发泄小游戏' },
]

function LaunchNoticeModal({
  onContinue,
  onNeverShowAgain,
  onExit,
}: {
  onContinue: () => void
  onNeverShowAgain: () => void
  onExit: () => void
}) {
  return (
    <div className="modal-backdrop" role="presentation">
      <div
        aria-describedby="launch-notice-description"
        aria-modal="true"
        className="modal-card"
        role="dialog"
      >
        <p className="eyebrow">使用前提示</p>
        <h2>本程序是本地优先的免费开源测评工具。</h2>
        <div id="launch-notice-description" className="modal-card__body">
          <p>在继续前，请先明确下面的边界：</p>
          <ul className="plain-list">
            <li>本程序不会向作者服务器上传或分析你的信息。</li>
            <li>题目、Prompt、聊天内容和评分请求会发送到你自己配置的第三方 AI 服务商。</li>
            <li>本程序完全免费开源。</li>
            <li>产生的费用全部来自你自己调用模型产生的 Token 消耗。</li>
            <li>作者不对程序产生的后果负责。</li>
          </ul>
          <p className="supporting-copy">
            继续使用即表示你已知晓以上边界与风险。当前工程骨架阶段，“以后不再弹出”只在本次会话内生效。
          </p>
        </div>
        <div className="footer-actions">
          <button className="button button--primary" onClick={onContinue} type="button">
            本次继续
          </button>
          <button
            className="button button--secondary"
            onClick={onNeverShowAgain}
            type="button"
          >
            以后不再弹出
          </button>
          <button className="button button--ghost" onClick={onExit} type="button">
            退出程序
          </button>
        </div>
      </div>
    </div>
  )
}

function BootView({ runtimeLabel }: { runtimeLabel: string }) {
  return (
    <div className="boot-screen">
      <div className="boot-screen__card">
        <p className="eyebrow">Prompt IQ Test</p>
        <h1>正在准备应用骨架</h1>
        <p>先读取启动摘要，再补加载模型配置摘要。</p>
        <p className="supporting-copy">{runtimeLabel}</p>
      </div>
    </div>
  )
}

function BootFailureView({
  message,
  onReload,
  onExit,
}: {
  message: string
  onReload: () => void
  onExit: () => void
}) {
  return (
    <div className="boot-screen">
      <div className="boot-screen__card boot-screen__card--danger">
        <p className="eyebrow">启动失败</p>
        <h1>当前无法安全完成应用启动</h1>
        <p>{message}</p>
        <div className="footer-actions">
          <button className="button button--primary" onClick={onReload} type="button">
            重新加载
          </button>
          <button className="button button--ghost" onClick={onExit} type="button">
            退出程序
          </button>
        </div>
      </div>
    </div>
  )
}

function AppShell() {
  const runtimeLabel = isTauriRuntime() ? 'Tauri 桌面运行时' : '浏览器 Mock'
  const [status, setStatus] = useState<BootStatus>('loading')
  const [bootstrap, setBootstrap] = useState<AppBootstrap>(mockBootstrap)
  const [profiles, setProfiles] = useState<SavedProfilesSummary>(mockProfiles)
  const [bootWarnings, setBootWarnings] = useState<CommandWarning[]>([])
  const [profileWarnings, setProfileWarnings] = useState<CommandWarning[]>([])
  const [launchNoticeOpen, setLaunchNoticeOpen] = useState(false)
  const [bootErrorMessage, setBootErrorMessage] = useState('应用启动失败。')

  useEffect(() => {
    let isActive = true

    async function load() {
      setStatus('loading')
      setBootErrorMessage('应用启动失败。')

      try {
        const bootResult = await getAppBootstrap()
        if (!isActive) {
          return
        }

        setBootstrap(bootResult.data)
        setBootWarnings(bootResult.warnings)
        setLaunchNoticeOpen(bootResult.data.settingsSummary.showLaunchNotice)

        try {
          const profileResult = await getSavedProfilesSummary()
          if (!isActive) {
            return
          }

          setProfiles(profileResult.data)
          setProfileWarnings(profileResult.warnings)
        } catch (error) {
          if (!isActive) {
            return
          }

          setProfiles(mockProfiles)
          setProfileWarnings([
            {
              code: 'PROFILES_FALLBACK_ACTIVE',
              message: error instanceof Error
                ? `模型配置摘要读取失败，当前使用 mock 数据。${error.message}`
                : '模型配置摘要读取失败，当前使用 mock 数据。',
            },
          ])
        }

        setStatus('ready')
      } catch (error) {
        if (!isActive) {
          return
        }

        setBootErrorMessage(
          error instanceof Error ? error.message : '应用启动失败。',
        )
        setStatus('failed')
      }
    }

    void load()

    return () => {
      isActive = false
    }
  }, [])

  function handleSettingsSaved(settingsSummary: SettingsSummary) {
    setBootstrap(prev => ({
      ...prev,
      settingsSummary,
    }))
  }

  function handleProfilesSaved(profilesSummary: SavedProfilesSummary) {
    setProfiles(profilesSummary)
  }

  const shellClassName = useMemo(() => {
    return [
      'app-shell',
      `app-shell--${bootstrap.settingsSummary.themeColor}`,
      `app-shell--${bootstrap.settingsSummary.fontSize}`,
    ].join(' ')
  }, [bootstrap.settingsSummary.fontSize, bootstrap.settingsSummary.themeColor])

  const allWarnings = [...bootWarnings, ...profileWarnings]

  function handleExit() {
    window.close()
  }

  if (status === 'loading') {
    return <BootView runtimeLabel={runtimeLabel} />
  }

  if (status === 'failed') {
    return (
      <BootFailureView
        message={bootErrorMessage}
        onReload={() => window.location.reload()}
        onExit={handleExit}
      />
    )
  }

  return (
    <div className={shellClassName}>
      <aside className="side-nav">
        <div className="side-nav__brand">
          <p className="eyebrow">Prompt IQ Test</p>
          <h1>本地优先 AI 能力测评</h1>
          <p>普通用户看会不会用 AI，进阶用户看能不能高质量驱动 LLM/Agent。</p>
        </div>

        <nav className="side-nav__links">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              className={({ isActive }) =>
                isActive ? 'nav-link nav-link--active' : 'nav-link'
              }
              end={item.end}
              to={item.to}
            >
              {item.label}
            </NavLink>
          ))}
        </nav>

        <div className="side-nav__footer">
          <span className="meta-pill">{runtimeLabel}</span>
          <span className="meta-pill">v{bootstrap.appMeta.version}</span>
        </div>
      </aside>

      <main className="workspace">
        {allWarnings.length > 0 ? (
          <section className="warning-ribbon">
            {allWarnings.map((warning) => (
              <p key={`${warning.code}-${warning.message}`}>
                <strong>{warning.code}</strong> {warning.message}
              </p>
            ))}
          </section>
        ) : null}

        <Routes>
          <Route
            path="/"
            element={
              <HomePage
                bootstrap={bootstrap}
                profiles={profiles}
                runtimeLabel={runtimeLabel}
              />
            }
          />
          <Route
            path="/assess/setup"
            element={
              <AssessmentSetupPage bootstrap={bootstrap} profiles={profiles} />
            }
          />
          <Route path="/assess/static" element={<StaticAssessmentPage />} />
          <Route path="/assess/dynamic" element={<DynamicAssessmentPage />} />
          <Route path="/result" element={<ResultReportPage />} />
          <Route path="/history" element={<HistoryListPage />} />
          <Route path="/history/:sessionId" element={<HistoryDetailPage />} />
          <Route
            path="/settings"
            element={
              <SettingsPage
                bootstrap={bootstrap}
                profiles={profiles}
                onSettingsSaved={handleSettingsSaved}
                onProfilesSaved={handleProfilesSaved}
              />
            }
          />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/plugins" element={<PluginMarketPage />} />
          <Route path="/vent" element={<VentGamePage />} />
          <Route path="*" element={<NotFoundPage />} />
        </Routes>
      </main>

      {launchNoticeOpen ? (
        <LaunchNoticeModal
          onContinue={() => setLaunchNoticeOpen(false)}
          onExit={handleExit}
          onNeverShowAgain={() => setLaunchNoticeOpen(false)}
        />
      ) : null}
    </div>
  )
}

function App() {
  return (
    <HashRouter>
      <AppShell />
    </HashRouter>
  )
}

export default App
