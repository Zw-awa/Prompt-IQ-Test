import { z } from 'zod'

import {
  mockBootstrap,
  mockProfiles,
  type AppBootstrap,
  type SavedProfilesSummary,
} from '../data/mock'

const appBootstrapSchema = z.object({
  appMeta: z.object({
    appName: z.string(),
    version: z.string(),
    license: z.string(),
  }),
  settingsSummary: z.object({
    themeColor: z.enum(['teal', 'amber', 'slate']),
    fontSize: z.enum(['small', 'medium', 'large']),
    showLaunchNotice: z.boolean(),
    defaultAdvancedEvaluationContext: z.boolean(),
    defaultMarkdownDirectory: z.string().nullable(),
  }),
  profileAvailability: z.object({
    executorConfigured: z.boolean(),
    evaluatorConfigured: z.boolean(),
    generatorConfigured: z.boolean(),
  }),
  historySummary: z.object({
    completedCount: z.number(),
    latestCompletedAt: z.string().nullable(),
  }),
})

const modelProfileSummarySchema = z.object({
  enabled: z.boolean(),
  baseUrl: z.string(),
  model: z.string(),
  hasApiKey: z.boolean(),
  apiKeyStorageMode: z.enum(['plain', 'encrypted', 'plain_fallback']),
  temperature: z.number(),
  maxTokens: z.number().nullable(),
  topP: z.number(),
  timeoutMs: z.number(),
  updatedAt: z.string(),
})

const savedProfilesSummarySchema = z.object({
  executor: modelProfileSummarySchema.nullable(),
  evaluator: modelProfileSummarySchema.nullable(),
  generator: z.object({
    source: z.enum(['evaluator', 'dedicated']),
    profile: modelProfileSummarySchema.nullable(),
  }),
})

type InvokeArgs = Record<string, unknown>

export function isTauriRuntime() {
  if (typeof window === 'undefined') {
    return false
  }

  return '__TAURI_INTERNALS__' in window
}

async function invokeCommand<T>(
  command: string,
  args: InvokeArgs,
  schema: z.ZodType<T>,
  fallback: T,
) {
  if (!isTauriRuntime()) {
    return fallback
  }

  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const response = await invoke<T>(command, args)
    return schema.parse(response)
  } catch (error) {
    console.warn(`[tauri] command failed: ${command}`, error)
    return fallback
  }
}

export function getAppBootstrap(): Promise<AppBootstrap> {
  return invokeCommand(
    'get_app_bootstrap',
    {},
    appBootstrapSchema,
    mockBootstrap,
  )
}

export function getSavedProfilesSummary(): Promise<SavedProfilesSummary> {
  return invokeCommand(
    'get_saved_profiles_summary',
    {},
    savedProfilesSummarySchema,
    mockProfiles,
  )
}
