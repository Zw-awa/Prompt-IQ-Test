import { z } from 'zod'

import {
  mockBootstrap,
  mockProfiles,
  type AppBootstrap,
  type SavedProfilesSummary,
} from '../data/mock'

const commandWarningSchema = z.object({
  code: z.string(),
  message: z.string(),
})

const commandErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
  severity: z.enum(['info', 'warning', 'error', 'fatal']),
  recoverable: z.boolean(),
  action: z.enum([
    'none',
    'retry_same_action',
    'go_to_settings',
    'reload_current_page',
    'back_to_home',
    'close_dialog',
  ]),
})

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

const settingsSummarySchema = z.object({
  themeColor: z.enum(['teal', 'amber', 'slate']),
  fontSize: z.enum(['small', 'medium', 'large']),
  showLaunchNotice: z.boolean(),
  defaultAdvancedEvaluationContext: z.boolean(),
  defaultMarkdownDirectory: z.string().nullable(),
})

const emptySchema = z.null()

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

const profileSchema = z.object({
  enabled: z.boolean(),
  baseUrl: z.string(),
  model: z.string(),
  apiKeyStorageMode: z.enum(['plain', 'encrypted', 'plain_fallback']),
  apiKey: z.string().nullable(),
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

const clearChatMessagesResultSchema = z.object({
  deletedMessageCount: z.number(),
  affectedSessionCount: z.number(),
})

const clearQuestionCacheResultSchema = z.object({
  deletedSessionCount: z.number(),
  deletedQuestionCacheEntryCount: z.number(),
})

const questionBankSummarySchema = z.object({
  totalQuestions: z.number(),
  staticQuestions: z.number(),
  dynamicQuestions: z.number(),
  byTaskType: z.record(z.number()),
})

const questionSchema = z.object({
  id: z.string(),
  source: z.enum(['builtin', 'ai_generated']),
  assessmentMode: z.enum(['static', 'dynamic']),
  taskType: z.enum(['daily_qa', 'writing', 'coding_debug', 'data_table', 'planning_analysis', 'agent_workflow']),
  scopeFit: z.array(z.enum(['fun', 'full'])),
  title: z.string(),
  versionInfo: z.object({
    questionSchemaVersion: z.string(),
    questionVersion: z.string(),
    rubricVersion: z.string(),
    generatorVersion: z.string().nullable(),
  }),
  visiblePrompt: z.object({
    task: z.string(),
    requirements: z.array(z.string()),
  }),
  hiddenReference: z.object({
    dimensionFocus: z.object({
      clarityOutput: z.number(),
      structuring: z.number(),
      contextConstraints: z.number(),
      gapRisk: z.number(),
      iterationAcceptance: z.number(),
    }),
    idealElements: z.array(z.string()),
    completionCriteria: z.array(z.string()),
    notesForEvaluator: z.string(),
    taskDoneSignals: z.array(z.string()).optional(),
  }),
})

type InvokeArgs = Record<string, unknown>

export type CommandWarning = z.infer<typeof commandWarningSchema>
export type CommandError = z.infer<typeof commandErrorSchema>

export type Profile = z.infer<typeof profileSchema>
export type SettingsSummary = z.infer<typeof settingsSummarySchema>
export type QuestionBankSummary = z.infer<typeof questionBankSummarySchema>
export type Question = z.infer<typeof questionSchema>

export type CommandEnvelope<T> =
  | {
      ok: true
      data: T
      warnings: CommandWarning[]
    }
  | {
      ok: false
      error: CommandError
    }

export type CommandResult<T> = {
  data: T
  warnings: CommandWarning[]
  source: 'tauri' | 'mock'
}

export function isTauriRuntime() {
  if (typeof window === 'undefined') {
    return false
  }

  return '__TAURI_INTERNALS__' in window
}

function buildEnvelopeSchema<T>(dataSchema: z.ZodType<T>) {
  return z.discriminatedUnion('ok', [
    z.object({
      ok: z.literal(true),
      data: dataSchema,
      warnings: z.array(commandWarningSchema),
    }),
    z.object({
      ok: z.literal(false),
      error: commandErrorSchema,
    }),
  ])
}

async function invokeCommand<T>(
  command: string,
  args: InvokeArgs,
  schema: z.ZodType<T>,
  fallback: T,
): Promise<CommandResult<T>> {
  if (!isTauriRuntime()) {
    return {
      data: fallback,
      warnings: [],
      source: 'mock',
    }
  }

  const { invoke } = await import('@tauri-apps/api/core')
  const response = await invoke<CommandEnvelope<T>>(command, args)
  const envelope = buildEnvelopeSchema(schema).parse(response)

  if (!envelope.ok) {
    throw new Error(
      `${command} failed: ${envelope.error.code} ${envelope.error.message}`,
    )
  }

  return {
    data: envelope.data,
    warnings: envelope.warnings,
    source: 'tauri',
  }
}

export function getAppBootstrap(): Promise<CommandResult<AppBootstrap>> {
  return invokeCommand(
    'get_app_bootstrap',
    {},
    appBootstrapSchema,
    mockBootstrap,
  )
}

export function getSavedProfilesSummary(): Promise<CommandResult<SavedProfilesSummary>> {
  return invokeCommand(
    'get_saved_profiles_summary',
    {},
    savedProfilesSummarySchema,
    mockProfiles,
  )
}

export function saveSettings(
  themeColor: string,
  fontSize: string,
  showLaunchNotice: boolean,
  defaultAdvancedEvaluationContext: boolean,
  defaultMarkdownDirectory: string | null,
): Promise<CommandResult<SettingsSummary>> {
  return invokeCommand(
    'save_settings',
    {
      themeColor,
      fontSize,
      showLaunchNotice,
      defaultAdvancedEvaluationContext,
      defaultMarkdownDirectory,
    },
    settingsSummarySchema,
    {
      themeColor: themeColor as 'teal' | 'amber' | 'slate',
      fontSize: fontSize as 'small' | 'medium' | 'large',
      showLaunchNotice,
      defaultAdvancedEvaluationContext,
      defaultMarkdownDirectory,
    },
  )
}

export function saveProfiles(
  executor: Profile,
  evaluator: Profile,
  generatorOverride: Profile,
): Promise<CommandResult<SavedProfilesSummary>> {
  return invokeCommand(
    'save_profiles',
    {
      executor,
      evaluator,
      generator_override: generatorOverride,
    },
    savedProfilesSummarySchema,
    mockProfiles,
  )
}

export function clearSavedProfiles(): Promise<CommandResult<null>> {
  return invokeCommand(
    'clear_saved_profiles',
    {},
    emptySchema,
    null,
  )
}

export function resetSettingsToDefault(): Promise<CommandResult<SettingsSummary>> {
  return invokeCommand(
    'reset_settings_to_default',
    {},
    settingsSummarySchema,
    mockBootstrap.settingsSummary,
  )
}

export function clearChatMessages(): Promise<CommandResult<{ deletedMessageCount: number, affectedSessionCount: number }>> {
  return invokeCommand(
    'clear_chat_messages',
    {},
    clearChatMessagesResultSchema,
    { deletedMessageCount: 0, affectedSessionCount: 0 },
  )
}

export function clearQuestionCache(): Promise<CommandResult<{ deletedSessionCount: number, deletedQuestionCacheEntryCount: number }>> {
  return invokeCommand(
    'clear_question_cache',
    {},
    clearQuestionCacheResultSchema,
    { deletedSessionCount: 0, deletedQuestionCacheEntryCount: 0 },
  )
}

export function getQuestionBankSummary(): Promise<CommandResult<QuestionBankSummary>> {
  return invokeCommand(
    'get_question_bank_summary',
    {},
    questionBankSummarySchema,
    { totalQuestions: 0, staticQuestions: 0, dynamicQuestions: 0, byTaskType: {} },
  )
}

export function drawQuestions(
  assessmentMode: 'static' | 'dynamic',
  scope: 'fun' | 'full',
  count: number,
  taskTypeFilter?: string,
): Promise<CommandResult<Question[]>> {
  return invokeCommand(
    'draw_questions',
    {
      assessment_mode: assessmentMode,
      scope,
      count,
      task_type_filter: taskTypeFilter,
    },
    z.array(questionSchema),
    [],
  )
}

// ==================== WP-06 Static Assessment Types ====================

export const modelProfileSummarySchema = z.object({
  enabled: z.boolean(),
  baseUrl: z.string(),
  model: z.string(),
  hasApiKey: z.boolean(),
  apiKeyStorageMode: z.string(),
  temperature: z.number(),
  maxTokens: z.number().nullable(),
  topP: z.number(),
  timeoutMs: z.number(),
  updatedAt: z.string(),
})

export const sessionSummarySchema = z.object({
  sessionId: z.string(),
  scope: z.string(),
  interactionMode: z.string(),
  questionSource: z.string(),
  status: z.string(),
  questionCount: z.number(),
  effectiveRoundCount: z.number(),
  advancedEvaluationContext: z.boolean(),
  staticStrategy: z.string().nullable(),
  executorModelSnapshot: modelProfileSummarySchema.nullable(),
  evaluatorModelSnapshot: modelProfileSummarySchema,
  generatorModelSnapshot: modelProfileSummarySchema.nullable(),
  createdAt: z.string(),
})

export const startAssessmentSuccessSchema = z.object({
  entryMode: z.string(),
  session: sessionSummarySchema,
  initialTaskReady: z.boolean(),
})

export type StartAssessmentSuccess = z.infer<typeof startAssessmentSuccessSchema>

export const taskProgressSchema = z.object({
  currentIndex: z.number(),
  totalCount: z.number(),
  completedCount: z.number(),
  remainingCount: z.number(),
})

export type TaskProgress = z.infer<typeof taskProgressSchema>

export const staticTaskDataSchema = z.object({
  taskId: z.string(),
  sequenceNo: z.number(),
  title: z.string(),
  task: z.string(),
  requirements: z.array(z.string()),
})

export type StaticTaskData = z.infer<typeof staticTaskDataSchema>

export const staticTaskPresentationSchema = z.object({
  state: z.string(),
  task: staticTaskDataSchema.nullable(),
  progress: taskProgressSchema,
})

export type StaticTaskPresentation = z.infer<typeof staticTaskPresentationSchema>

export const scoredTaskSummarySchema = z.object({
  taskId: z.string(),
  finalScore: z.number(),
  executorOutputPreview: z.string().nullable(),
})

export type ScoredTaskSummary = z.infer<typeof scoredTaskSummarySchema>

export const coreScoresSummarySchema = z.object({
  clarityOutput: z.number(),
  structuring: z.number(),
  contextConstraints: z.number(),
  gapRisk: z.number(),
  iterationAcceptance: z.number(),
})

export type CoreScoresSummary = z.infer<typeof coreScoresSummarySchema>

export const confidenceSummarySchema = z.object({
  level: z.string(),
  note: z.string(),
})

export type ConfidenceSummary = z.infer<typeof confidenceSummarySchema>

export const finalReportSummarySchema = z.object({
  sessionId: z.string(),
  scope: z.string(),
  interactionMode: z.string(),
  staticStrategy: z.string().nullable(),
  totalScore: z.number(),
  coreScores: coreScoresSummarySchema,
  strengths: z.array(z.string()),
  issues: z.array(z.string()),
  suggestions: z.array(z.string()),
  summary: z.string(),
  confidence: confidenceSummarySchema,
  questionCount: z.number(),
  effectiveRoundCount: z.number(),
})

export type FinalReportSummary = z.infer<typeof finalReportSummarySchema>

export const submitStaticPromptSuccessSchema = z.object({
  nextAction: z.string(),
  scoredTask: scoredTaskSummarySchema.nullable(),
  progress: taskProgressSchema.nullable(),
  finalReport: finalReportSummarySchema.nullable(),
})

export type SubmitStaticPromptSuccess = z.infer<typeof submitStaticPromptSuccessSchema>

// ==================== WP-06 Command Wrappers ====================

export function startAssessmentSession(
  scope: string,
  interactionMode: string,
  questionSource: string,
  staticStrategy: string | null,
  advancedEvaluationContext: boolean,
  taskType?: string,
  generationConstraints?: string,
  runtimeModelOverrides?: {
    executorModelOverride?: string | null
    evaluatorModelOverride?: string | null
    generatorModelOverride?: string | null
  } | null,
): Promise<CommandResult<StartAssessmentSuccess>> {
  return invokeCommand(
    'start_assessment_session',
    {
      scope,
      interaction_mode: interactionMode,
      question_source: questionSource,
      static_strategy: staticStrategy,
      advanced_evaluation_context: advancedEvaluationContext,
      task_type: taskType ?? null,
      generation_constraints: generationConstraints ?? null,
      runtime_model_overrides: runtimeModelOverrides ?? null,
    },
    startAssessmentSuccessSchema,
    {
      entryMode: 'static',
      session: {
        sessionId: 'mock-session-id',
        scope,
        interactionMode,
        questionSource,
        status: 'running',
        questionCount: 5,
        effectiveRoundCount: 0,
        advancedEvaluationContext,
        staticStrategy,
        executorModelSnapshot: null,
        evaluatorModelSnapshot: {
          enabled: true,
          baseUrl: 'https://api.openai.com/v1',
          model: 'gpt-4.1',
          hasApiKey: true,
          apiKeyStorageMode: 'plain',
          temperature: 0.2,
          maxTokens: 4096,
          topP: 1,
          timeoutMs: 60000,
          updatedAt: '2026-04-01T00:00:00Z',
        },
        generatorModelSnapshot: null,
        createdAt: new Date().toISOString(),
      },
      initialTaskReady: true,
    },
  )
}

export function loadNextStaticTask(
  sessionId: string,
): Promise<CommandResult<StaticTaskPresentation>> {
  return invokeCommand(
    'load_next_static_task',
    { session_id: sessionId },
    staticTaskPresentationSchema,
    {
      state: 'ready',
      task: {
        taskId: 'mock-task-id',
        sequenceNo: 1,
        title: '把模糊任务写成可执行 Prompt',
        task: '你要让 AI 帮你生成一份桌面端设置页文案规范，请写出一段高质量 Prompt。',
        requirements: ['要求 AI 输出 Markdown', '必须说明区块顺序', '要包含禁用与异常提示规则'],
      },
      progress: {
        currentIndex: 1,
        totalCount: 5,
        completedCount: 0,
        remainingCount: 5,
      },
    },
  )
}

export function submitStaticPrompt(
  sessionId: string,
  taskId: string,
  userPrompt: string,
): Promise<CommandResult<SubmitStaticPromptSuccess>> {
  return invokeCommand(
    'submit_static_prompt',
    { session_id: sessionId, task_id: taskId, user_prompt: userPrompt },
    submitStaticPromptSuccessSchema,
    {
      nextAction: 'continue',
      scoredTask: {
        taskId,
        finalScore: 78,
        executorOutputPreview: null,
      },
      progress: {
        currentIndex: 1,
        totalCount: 5,
        completedCount: 1,
        remainingCount: 4,
      },
      finalReport: null,
    },
  )
}
