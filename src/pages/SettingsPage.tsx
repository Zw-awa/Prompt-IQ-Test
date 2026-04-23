import { useState } from 'react'
import { Link } from 'react-router-dom'

import { PageSection } from '../components/PageSection'
import { StatusPill } from '../components/StatusPill'
import type { AppBootstrap, SavedProfilesSummary } from '../data/mock'
import { mockProfiles } from '../data/mock'
import {
  saveSettings,
  saveProfiles,
  clearSavedProfiles,
  resetSettingsToDefault,
  type Profile,
  type SettingsSummary,
} from '../lib/tauri'


type SettingsPageProps = {
  bootstrap: AppBootstrap
  profiles: SavedProfilesSummary
  onSettingsSaved?: (settings: SettingsSummary) => void
  onProfilesSaved?: (profiles: SavedProfilesSummary) => void
}

export function SettingsPage({
  bootstrap,
  profiles,
  onSettingsSaved,
  onProfilesSaved,
}: SettingsPageProps) {
  const [themeColor, setThemeColor] = useState(
    bootstrap.settingsSummary.themeColor,
  )
  const [fontSize, setFontSize] = useState(bootstrap.settingsSummary.fontSize)
  const [showLaunchNotice, setShowLaunchNotice] = useState(
    bootstrap.settingsSummary.showLaunchNotice,
  )
  const [preferEncryptedApiKey, setPreferEncryptedApiKey] = useState(false)
  const [defaultMarkdownDirectory, setDefaultMarkdownDirectory] = useState(
    bootstrap.settingsSummary.defaultMarkdownDirectory ?? '',
  )
  const [isSaving, setIsSaving] = useState(false)
  const [saveError, setSaveError] = useState<string | null>(null)
  const [saveSuccess, setSaveSuccess] = useState(false)

  async function handleSave() {
    setIsSaving(true)
    setSaveError(null)
    setSaveSuccess(false)

    try {
      // Save settings
      const settingsResult = await saveSettings(
        themeColor,
        fontSize,
        showLaunchNotice,
        bootstrap.settingsSummary.defaultAdvancedEvaluationContext,
        defaultMarkdownDirectory || null,
      )
      onSettingsSaved?.(settingsResult.data)

      // Save profiles - construct Profile objects from summaries
      // For now, we'll pass minimal profiles with null API keys to preserve existing
      // In a real implementation, we would need full profile editing UI
      const executorProfile: Profile = profiles.executor ? {
        enabled: profiles.executor.enabled,
        baseUrl: profiles.executor.baseUrl,
        model: profiles.executor.model,
        apiKeyStorageMode: profiles.executor.apiKeyStorageMode,
        apiKey: null, // preserve existing
        temperature: profiles.executor.temperature,
        maxTokens: profiles.executor.maxTokens,
        topP: profiles.executor.topP,
        timeoutMs: profiles.executor.timeoutMs,
        updatedAt: new Date().toISOString(),
      } : {
        enabled: false,
        baseUrl: '',
        model: '',
        apiKeyStorageMode: 'plain',
        apiKey: null,
        temperature: 0.7,
        maxTokens: null,
        topP: 1.0,
        timeoutMs: 60000,
        updatedAt: new Date().toISOString(),
      }

      const evaluatorProfile: Profile = profiles.evaluator ? {
        enabled: profiles.evaluator.enabled,
        baseUrl: profiles.evaluator.baseUrl,
        model: profiles.evaluator.model,
        apiKeyStorageMode: profiles.evaluator.apiKeyStorageMode,
        apiKey: null,
        temperature: profiles.evaluator.temperature,
        maxTokens: profiles.evaluator.maxTokens,
        topP: profiles.evaluator.topP,
        timeoutMs: profiles.evaluator.timeoutMs,
        updatedAt: new Date().toISOString(),
      } : {
        enabled: false,
        baseUrl: '',
        model: '',
        apiKeyStorageMode: 'plain',
        apiKey: null,
        temperature: 0.7,
        maxTokens: null,
        topP: 1.0,
        timeoutMs: 60000,
        updatedAt: new Date().toISOString(),
      }

      const generatorProfile: Profile = profiles.generator.profile ? {
        enabled: profiles.generator.profile.enabled,
        baseUrl: profiles.generator.profile.baseUrl,
        model: profiles.generator.profile.model,
        apiKeyStorageMode: profiles.generator.profile.apiKeyStorageMode,
        apiKey: null,
        temperature: profiles.generator.profile.temperature,
        maxTokens: profiles.generator.profile.maxTokens,
        topP: profiles.generator.profile.topP,
        timeoutMs: profiles.generator.profile.timeoutMs,
        updatedAt: new Date().toISOString(),
      } : {
        enabled: false,
        baseUrl: '',
        model: '',
        apiKeyStorageMode: 'plain',
        apiKey: null,
        temperature: 0.7,
        maxTokens: null,
        topP: 1.0,
        timeoutMs: 60000,
        updatedAt: new Date().toISOString(),
      }

      const profilesResult = await saveProfiles(
        executorProfile,
        evaluatorProfile,
        generatorProfile,
      )
      onProfilesSaved?.(profilesResult.data)

      setSaveSuccess(true)
    } catch (error) {
      setSaveError(error instanceof Error ? error.message : '保存失败')
    } finally {
      setIsSaving(false)
    }
  }

  const [dataManagementLoading, setDataManagementLoading] = useState<string | null>(null)
  const [dataManagementError, setDataManagementError] = useState<string | null>(null)

  async function handleClearChatHistory() {
    if (!confirm('确定要清空所有聊天记录吗？此操作不可恢复。')) {
      return
    }
    setDataManagementLoading('clearChatHistory')
    setDataManagementError(null)
    try {
      // TODO: Implement backend command for clearing chat history
      await new Promise(resolve => setTimeout(resolve, 500)) // Simulate async
      alert('清空聊天记录功能尚未实现')
    } catch (error) {
      setDataManagementError(error instanceof Error ? error.message : '操作失败')
    } finally {
      setDataManagementLoading(null)
    }
  }

  async function handleClearQuestionCache() {
    if (!confirm('确定要清空题目缓存吗？此操作将删除所有未完成的测评会话和生成的题目缓存，不可恢复。')) {
      return
    }
    setDataManagementLoading('clearQuestionCache')
    setDataManagementError(null)
    try {
      // TODO: Implement backend command for clearing question cache
      await new Promise(resolve => setTimeout(resolve, 500)) // Simulate async
      alert('清空题目缓存功能尚未实现')
    } catch (error) {
      setDataManagementError(error instanceof Error ? error.message : '操作失败')
    } finally {
      setDataManagementLoading(null)
    }
  }

  async function handleClearModelConfig() {
    if (!confirm('确定要清空所有模型配置吗？此操作将删除所有已保存的API密钥和模型设置，不可恢复。')) {
      return
    }
    setDataManagementLoading('clearModelConfig')
    setDataManagementError(null)
    try {
      const result = await clearSavedProfiles()
      if (result.warnings.length > 0) {
        console.warn('Warnings from clearSavedProfiles:', result.warnings)
      }
      alert('模型配置已清空')
      onProfilesSaved?.(mockProfiles) // Reset profiles to mock
    } catch (error) {
      setDataManagementError(error instanceof Error ? error.message : '操作失败')
    } finally {
      setDataManagementLoading(null)
    }
  }

  async function handleResetSettings() {
    if (!confirm('确定要恢复默认设置吗？此操作将重置所有外观和隐私设置，不可恢复。')) {
      return
    }
    setDataManagementLoading('resetSettings')
    setDataManagementError(null)
    try {
      const result = await resetSettingsToDefault()
      onSettingsSaved?.(result.data)
      alert('设置已恢复为默认值')
    } catch (error) {
      setDataManagementError(error instanceof Error ? error.message : '操作失败')
    } finally {
      setDataManagementLoading(null)
    }
  }

  return (
    <div className="page-stack">
      <PageSection
        title="设置"
        description="管理本地模型配置、外观偏好与数据边界。当前批次只做界面骨架，不做真实持久化写入。"
        actions={
          <Link className="button button--ghost" to="/">
            返回
          </Link>
        }
      >
        <div className="stacked-alert stacked-alert--warning">
          <h3>当前仍是工程骨架</h3>
          <p>保存更改、数据清理和加密回退的真实后端链路会在后续批次补上。</p>
        </div>

        {saveError && (
          <div className="stacked-alert stacked-alert--danger">
            <h3>保存失败</h3>
            <p>{saveError}</p>
          </div>
        )}

        {saveSuccess && (
          <div className="stacked-alert stacked-alert--success">
            <h3>保存成功</h3>
            <p>设置已保存。</p>
          </div>
        )}
      </PageSection>

      <PageSection title="模型配置区" description="首版固定顺序：执行 AI -> 测评 AI -> 出题 AI。">
        <div className="card-grid card-grid--three">
          <article className="info-card">
            <div className="info-card__top">
              <h3>执行 AI</h3>
              <StatusPill
                label={profiles.executor ? '已配置' : '未配置'}
                tone={profiles.executor ? 'success' : 'warning'}
              />
            </div>
            <p>模型：{profiles.executor?.model ?? '未填写'}</p>
            <p>保存方式：{profiles.executor?.apiKeyStorageMode ?? '未知'}</p>
          </article>

          <article className="info-card">
            <div className="info-card__top">
              <h3>测评 AI</h3>
              <StatusPill
                label={profiles.evaluator ? '已配置' : '未配置'}
                tone={profiles.evaluator ? 'success' : 'warning'}
              />
            </div>
            <p>模型：{profiles.evaluator?.model ?? '未填写'}</p>
            <p>保存方式：{profiles.evaluator?.apiKeyStorageMode ?? '未知'}</p>
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
              当前模式：
              {profiles.generator.source === 'evaluator'
                ? '跟随测评 AI'
                : '独立 profile'}
            </p>
            <p>模型：{profiles.generator.profile?.model ?? '未单独配置'}</p>
          </article>
        </div>
      </PageSection>

      <PageSection title="外观区" description="首版先接主题色和字体大小两个基础项。">
        <div className="form-grid form-grid--two">
          <label className="field">
            <span>主题色</span>
            <select
              value={themeColor}
              onChange={(event) =>
                setThemeColor(event.target.value as typeof themeColor)
              }
            >
              <option value="teal">Teal</option>
              <option value="amber">Amber</option>
              <option value="slate">Slate</option>
            </select>
          </label>

          <label className="field">
            <span>字体大小</span>
            <select
              value={fontSize}
              onChange={(event) =>
                setFontSize(event.target.value as typeof fontSize)
              }
            >
              <option value="small">Small</option>
              <option value="medium">Medium</option>
              <option value="large">Large</option>
            </select>
          </label>
        </div>
      </PageSection>

      <PageSection title="隐私与存储区" description="明文 / 加密 / 加密回退状态后续会接真实环境检测。">
        <div className="form-grid form-grid--two">
          <label className="checkbox-field">
            <input
              type="checkbox"
              checked={showLaunchNotice}
              onChange={(event) => setShowLaunchNotice(event.target.checked)}
            />
            <span>启动时显示使用前提示</span>
          </label>

          <label className="checkbox-field">
            <input
              type="checkbox"
              checked={preferEncryptedApiKey}
              onChange={(event) =>
                setPreferEncryptedApiKey(event.target.checked)
              }
            />
            <span>优先使用加密保存 API Key</span>
          </label>

          <label className="field field--full">
            <span>默认 Markdown 导出目录</span>
            <input
              value={defaultMarkdownDirectory}
              onChange={(event) =>
                setDefaultMarkdownDirectory(event.target.value)
              }
              placeholder="未设置时使用系统默认目录"
            />
          </label>
        </div>
      </PageSection>

      <PageSection title="数据管理区" description="四个危险动作都先保留确认入口，不真正删除本地数据。">
        <div className="card-grid card-grid--two">
          <button
            className="button button--secondary"
            type="button"
            onClick={handleClearChatHistory}
            disabled={dataManagementLoading === 'clearChatHistory'}
          >
            {dataManagementLoading === 'clearChatHistory' ? '处理中...' : '清空聊天记录'}
          </button>
          <button
            className="button button--secondary"
            type="button"
            onClick={handleClearQuestionCache}
            disabled={dataManagementLoading === 'clearQuestionCache'}
          >
            {dataManagementLoading === 'clearQuestionCache' ? '处理中...' : '清空题目缓存'}
          </button>
          <button
            className="button button--secondary"
            type="button"
            onClick={handleClearModelConfig}
            disabled={dataManagementLoading === 'clearModelConfig'}
          >
            {dataManagementLoading === 'clearModelConfig' ? '处理中...' : '清空模型配置'}
          </button>
          <button
            className="button button--secondary"
            type="button"
            onClick={handleResetSettings}
            disabled={dataManagementLoading === 'resetSettings'}
          >
            {dataManagementLoading === 'resetSettings' ? '处理中...' : '恢复默认设置'}
          </button>
        </div>
        {dataManagementError && (
          <div className="stacked-alert stacked-alert--danger">
            <h3>操作失败</h3>
            <p>{dataManagementError}</p>
          </div>
        )}
      </PageSection>

      {dataManagementError && (
        <div className="stacked-alert stacked-alert--error">
          <h3>数据管理操作失败</h3>
          <p>{dataManagementError}</p>
        </div>
      )}

      {saveError && (
        <div className="stacked-alert stacked-alert--error">
          <h3>保存失败</h3>
          <p>{saveError}</p>
        </div>
      )}
      
      {saveSuccess && (
        <div className="stacked-alert stacked-alert--success">
          <h3>保存成功</h3>
          <p>设置已保存到本地。</p>
        </div>
      )}

      <PageSection title="底部操作区" description="首版保留统一保存入口，不做分区保存。">
        <div className="footer-actions">
          <button 
            className="button button--primary" 
            type="button"
            onClick={handleSave}
            disabled={isSaving}
          >
            {isSaving ? '保存中...' : '保存更改'}
          </button>
          <Link className="button button--ghost" to="/">
            取消并返回
          </Link>
        </div>
      </PageSection>
    </div>
  )
}
