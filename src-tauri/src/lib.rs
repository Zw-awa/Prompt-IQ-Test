use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use chrono::Utc;
use rusqlite::{Connection, params, OptionalExtension};
use question_bank::QuestionSchema;

mod db;
mod question_bank;
mod ai_client;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    None,
    RetrySameAction,
    GoToSettings,
    ReloadCurrentPage,
    BackToHome,
    CloseDialog,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommandWarning {
  code: String,
  message: String,
  severity: Severity,
  recoverable: bool,
  action: Action,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommandError {
  code: String,
  message: String,
  severity: Severity,
  recoverable: bool,
  action: Action,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommandSuccess<T>
where
  T: Serialize,
{
  ok: bool,
  data: T,
  warnings: Vec<CommandWarning>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Settings {
    schema_version: String,
    ui: SettingsUi,
    privacy: SettingsPrivacy,
    assessment: SettingsAssessment,
    export: SettingsExport,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsUi {
    theme_color: String,
    font_size: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsPrivacy {
    show_launch_notice: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsAssessment {
    default_advanced_evaluation_context: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SettingsExport {
    default_markdown_directory: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profiles {
    schema_version: String,
    executor: Profile,
    evaluator: Profile,
    generator_override: Profile,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    enabled: bool,
    base_url: String,
    model: String,
    api_key_storage_mode: String,
    api_key: Option<String>,
    temperature: f32,
    max_tokens: Option<u32>,
    top_p: f32,
    timeout_ms: u32,
    updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppMeta {
  app_name: String,
  version: String,
  license: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SettingsSummary {
  theme_color: String,
  font_size: String,
  show_launch_notice: bool,
  default_advanced_evaluation_context: bool,
  default_markdown_directory: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileAvailability {
  executor_configured: bool,
  evaluator_configured: bool,
  generator_configured: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistorySummary {
  completed_count: u32,
  latest_completed_at: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppBootstrap {
  app_meta: AppMeta,
  settings_summary: SettingsSummary,
  profile_availability: ProfileAvailability,
  history_summary: HistorySummary,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ModelProfileSummary {
  enabled: bool,
  base_url: String,
  model: String,
  has_api_key: bool,
  api_key_storage_mode: String,
  temperature: f32,
  max_tokens: Option<u32>,
  top_p: f32,
  timeout_ms: u32,
  updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeneratorProfileSummary {
  source: String,
  profile: Option<ModelProfileSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SavedProfilesSummary {
  executor: Option<ModelProfileSummary>,
  evaluator: Option<ModelProfileSummary>,
  generator: GeneratorProfileSummary,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClearChatMessagesResult {
  deleted_message_count: u32,
  affected_session_count: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClearQuestionCacheResult {
  deleted_session_count: u32,
  deleted_question_cache_entry_count: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QuestionBankSummary {
  total_questions: u32,
  static_questions: u32,
  dynamic_questions: u32,
  by_task_type: std::collections::HashMap<String, u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrawQuestionsRequest {
  assessment_mode: String,
  scope: String,
  count: u32,
  task_type_filter: Option<String>,
}

// ==================== WP-06 Assessment Session Structures ====================

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssessmentStartRequest {
    scope: String,
    interaction_mode: String,
    question_source: String,
    static_strategy: Option<String>,
    dynamic_end_strategy: Option<String>,
    fixed_round_limit: Option<i32>,
    task_type: Option<String>,
    generation_constraints: Option<String>,
    advanced_evaluation_context: bool,
    runtime_model_overrides: Option<RuntimeModelOverrides>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeModelOverrides {
    executor_model_override: Option<String>,
    evaluator_model_override: Option<String>,
    generator_model_override: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StartAssessmentSuccess {
    entry_mode: String,
    session: SessionSummary,
    initial_task_ready: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionSummary {
    session_id: String,
    scope: String,
    interaction_mode: String,
    question_source: String,
    status: String,
    question_count: i32,
    effective_round_count: i32,
    advanced_evaluation_context: bool,
    static_strategy: Option<String>,
    executor_model_snapshot: Option<ModelProfileSummary>,
    evaluator_model_snapshot: ModelProfileSummary,
    generator_model_snapshot: Option<ModelProfileSummary>,
    created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaticTaskPresentation {
    state: String,
    task: Option<StaticTaskData>,
    progress: TaskProgress,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaticTaskData {
    task_id: String,
    sequence_no: i32,
    title: String,
    task: String,
    requirements: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskProgress {
    current_index: i32,
    total_count: i32,
    completed_count: i32,
    remaining_count: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmitStaticPromptRequest {
    session_id: String,
    task_id: String,
    user_prompt: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmitStaticPromptSuccess {
    next_action: String,
    scored_task: Option<ScoredTaskSummary>,
    progress: Option<TaskProgress>,
    final_report: Option<FinalReportSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScoredTaskSummary {
    task_id: String,
    final_score: f64,
    executor_output_preview: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FinalReportSummary {
    session_id: String,
    scope: String,
    interaction_mode: String,
    static_strategy: Option<String>,
    total_score: f64,
    core_scores: CoreScoresSummary,
    strengths: Vec<String>,
    issues: Vec<String>,
    suggestions: Vec<String>,
    summary: String,
    confidence: ConfidenceSummary,
    question_count: i32,
    effective_round_count: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CoreScoresSummary {
    clarity_output: f64,
    structuring: f64,
    context_constraints: f64,
    gap_risk: f64,
    iteration_acceptance: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfidenceSummary {
    level: String,
    note: String,
}

// ==================== Static Assessment AI Evaluation ====================

fn build_evaluator_prompt(
    task: &db::AssessmentTask,
    user_prompt: &str,
    advanced_eval_context: bool,
) -> String {
    let core_dimensions = r#"Core Dimensions:
1. clarity_output (weight 30): How clearly does the user specify the expected output?
2. structuring (weight 25): How well does the user structure vague requests?
3. context_constraints (weight 20): How sufficient are the context and constraints?
4. gap_risk (weight 15): How well does the user identify missing conditions?
5. iteration_acceptance (weight 10): How open is the user to iteration and feedback?"#;

    let mut prompt = format!(
        r#"[RUN META]
promptTemplateVersion=eval-static-prompt-only-v1
rubricVersion=rubric-v1
schemaVersion=static-eval-v1
responseLanguage=zh-CN

[ASSESSMENT META]
mode=static
staticStrategy=prompt_only
scope=comprehensive
questionSource=builtin
taskType={}
advancedEvaluationContext={}

[VISIBLE TASK]
{}
"#,
        task.task_type,
        if advanced_eval_context { "true" } else { "false" },
        task.visible_task,
    );

    if let Ok(requirements) = serde_json::from_str::<Vec<String>>(&task.visible_requirements_json) {
        prompt.push_str("\nRequirements:\n");
        for req in &requirements {
            prompt.push_str(&format!("- {}\n", req));
        }
    }

    prompt.push_str(&format!(
        r#"

[USER SUBMISSION]
{}

[CORE DIMENSIONS]
{}

"#,
        user_prompt, core_dimensions,
    ));

    // Advanced context: detailed rubric + hidden reference
    if advanced_eval_context {
        prompt.push_str("[DETAILED RUBRIC]\n");
        prompt.push_str("Scoring Guide:\n");
        prompt.push_str("- 90-100: Exceptional. Clear, complete, well-structured.\n");
        prompt.push_str("- 70-89: Good. Mostly clear with minor gaps.\n");
        prompt.push_str("- 50-69: Average. Some clarity but notable omissions.\n");
        prompt.push_str("- 30-49: Below average. Significant gaps or confusion.\n");
        prompt.push_str("- 0-29: Poor. Unclear, incomplete, or off-target.\n\n");

        prompt.push_str("[HIDDEN REFERENCE]\n");
        prompt.push_str(&task.hidden_reference_json);
        prompt.push('\n');
    }

    prompt.push_str(r#"
[OUTPUT CONTRACT]
Return JSON only.
Use schemaVersion=static-eval-v1.
Use promptTemplateVersion=eval-static-prompt-only-v1.
Set staticStrategy=prompt_only.
Set executionOutcomeScore=null.
Compute finalScore = round(rubricScore * 0.85 + holisticScore * 0.15).
For each core dimension, provide rubric, holistic, and final.
For each subdimension item, include id, name, score, reason, and evidence[].
Natural-language fields summary / strengths / issues / suggestions must use Simplified Chinese. If uncertain, use Simplified Chinese.
"#);

    prompt
}

fn parse_evaluation_response(response_text: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str::<serde_json::Value>(response_text)
        .map_err(|e| format!("无法解析测评AI返回的JSON: {}", e))
}

fn calculate_final_score_from_evaluation(eval: &serde_json::Value) -> Result<f64, String> {
    let rubric_score = eval.get("rubricScore")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| "缺少 rubricScore".to_string())?;
    let holistic_score = eval.get("holisticScore")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| "缺少 holisticScore".to_string())?;
    let final_score = (rubric_score * 0.85 + holistic_score * 0.15).round();
    Ok(final_score)
}

fn extract_core_scores(eval: &serde_json::Value) -> Option<CoreScoresSummary> {
    let cs = eval.get("coreScores")?;
    Some(CoreScoresSummary {
        clarity_output: cs.get("clarity_output")?.get("final")?.as_f64()?,
        structuring: cs.get("structuring")?.get("final")?.as_f64()?,
        context_constraints: cs.get("context_constraints")?.get("final")?.as_f64()?,
        gap_risk: cs.get("gap_risk")?.get("final")?.as_f64()?,
        iteration_acceptance: cs.get("iteration_acceptance")?.get("final")?.as_f64()?,
    })
}

fn extract_string_array(eval: &serde_json::Value, key: &str) -> Vec<String> {
    eval.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|s| s.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

fn extract_confidence(eval: &serde_json::Value) -> ConfidenceSummary {
    let conf = eval.get("confidence");
    ConfidenceSummary {
        level: conf.and_then(|c| c.get("level")).and_then(|v| v.as_str()).unwrap_or("low").to_string(),
        note: conf.and_then(|c| c.get("note")).and_then(|v| v.as_str()).unwrap_or("").to_string(),
    }
}

fn shared_profile(model: &str) -> ModelProfileSummary {
  ModelProfileSummary {
    enabled: true,
    base_url: "https://api.openai.com/v1".into(),
    model: model.into(),
    has_api_key: true,
    api_key_storage_mode: "plain".into(),
    temperature: 0.3,
    max_tokens: None,
    top_p: 1.0,
    timeout_ms: 60000,
    updated_at: "2026-04-13T18:20:00+08:00".into(),
  }
}

#[tauri::command]
fn get_app_bootstrap() -> Result<CommandSuccess<AppBootstrap>, CommandError> {
    let settings = read_settings()?;
    let profiles = read_profiles()?;
    
    let executor_configured = profiles.executor.enabled && 
        !profiles.executor.base_url.is_empty() && 
        !profiles.executor.model.is_empty() && 
        profiles.executor.api_key.is_some();
    let evaluator_configured = profiles.evaluator.enabled && 
        !profiles.evaluator.base_url.is_empty() && 
        !profiles.evaluator.model.is_empty() && 
        profiles.evaluator.api_key.is_some();
    let generator_configured = if profiles.generator_override.enabled {
        !profiles.generator_override.base_url.is_empty() && 
        !profiles.generator_override.model.is_empty() && 
        profiles.generator_override.api_key.is_some()
    } else {
        evaluator_configured
    };
    
    // TODO: read from database, for now mock
    let history_summary = HistorySummary {
        completed_count: 0,
        latest_completed_at: None,
    };
    
    Ok(CommandSuccess {
        ok: true,
        data: AppBootstrap {
            app_meta: AppMeta {
                app_name: "Prompt IQ Test".into(),
                version: "0.1.0".into(),
                license: "Apache-2.0".into(),
            },
            settings_summary: SettingsSummary {
                theme_color: settings.ui.theme_color,
                font_size: settings.ui.font_size,
                show_launch_notice: settings.privacy.show_launch_notice,
                default_advanced_evaluation_context: settings.assessment.default_advanced_evaluation_context,
                default_markdown_directory: settings.export.default_markdown_directory,
            },
            profile_availability: ProfileAvailability {
                executor_configured,
                evaluator_configured,
                generator_configured,
            },
            history_summary,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn get_saved_profiles_summary() -> Result<CommandSuccess<SavedProfilesSummary>, CommandError> {
    let profiles = read_profiles()?;
    
    let executor_summary = if profiles.executor.enabled && 
        !profiles.executor.base_url.is_empty() && 
        !profiles.executor.model.is_empty() && 
        profiles.executor.api_key.is_some() {
        Some(ModelProfileSummary {
            enabled: profiles.executor.enabled,
            base_url: profiles.executor.base_url,
            model: profiles.executor.model,
            has_api_key: profiles.executor.api_key.is_some(),
            api_key_storage_mode: profiles.executor.api_key_storage_mode,
            temperature: profiles.executor.temperature,
            max_tokens: profiles.executor.max_tokens,
            top_p: profiles.executor.top_p,
            timeout_ms: profiles.executor.timeout_ms,
            updated_at: profiles.executor.updated_at,
        })
    } else {
        None
    };
    
    let evaluator_summary = if profiles.evaluator.enabled && 
        !profiles.evaluator.base_url.is_empty() && 
        !profiles.evaluator.model.is_empty() && 
        profiles.evaluator.api_key.is_some() {
        Some(ModelProfileSummary {
            enabled: profiles.evaluator.enabled,
            base_url: profiles.evaluator.base_url,
            model: profiles.evaluator.model,
            has_api_key: profiles.evaluator.api_key.is_some(),
            api_key_storage_mode: profiles.evaluator.api_key_storage_mode,
            temperature: profiles.evaluator.temperature,
            max_tokens: profiles.evaluator.max_tokens,
            top_p: profiles.evaluator.top_p,
            timeout_ms: profiles.evaluator.timeout_ms,
            updated_at: profiles.evaluator.updated_at,
        })
    } else {
        None
    };
    
    let generator_source = if profiles.generator_override.enabled {
        "dedicated".into()
    } else {
        "evaluator".into()
    };
    
    let generator_profile = if profiles.generator_override.enabled && 
        !profiles.generator_override.base_url.is_empty() && 
        !profiles.generator_override.model.is_empty() && 
        profiles.generator_override.api_key.is_some() {
        Some(ModelProfileSummary {
            enabled: profiles.generator_override.enabled,
            base_url: profiles.generator_override.base_url,
            model: profiles.generator_override.model,
            has_api_key: profiles.generator_override.api_key.is_some(),
            api_key_storage_mode: profiles.generator_override.api_key_storage_mode,
            temperature: profiles.generator_override.temperature,
            max_tokens: profiles.generator_override.max_tokens,
            top_p: profiles.generator_override.top_p,
            timeout_ms: profiles.generator_override.timeout_ms,
            updated_at: profiles.generator_override.updated_at,
        })
    } else {
        None
    };
    
    Ok(CommandSuccess {
        ok: true,
        data: SavedProfilesSummary {
            executor: executor_summary,
            evaluator: evaluator_summary,
            generator: GeneratorProfileSummary {
                source: generator_source,
                profile: generator_profile,
            },
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn save_profiles(
    executor: Profile,
    evaluator: Profile,
    generator_override: Profile,
) -> Result<CommandSuccess<SavedProfilesSummary>, CommandError> {
    // Read existing profiles to preserve API keys when null
    let mut existing = read_profiles()?;
    
    // Helper to merge profile with API key preservation logic
    fn merge_profile(new: &Profile, existing: &mut Profile) {
        existing.enabled = new.enabled;
        existing.base_url = new.base_url.clone();
        existing.model = new.model.clone();
        existing.api_key_storage_mode = new.api_key_storage_mode.clone();
        existing.temperature = new.temperature;
        existing.max_tokens = new.max_tokens;
        existing.top_p = new.top_p;
        existing.timeout_ms = new.timeout_ms;
        
        // Handle API key according to contract
        match &new.api_key {
            Some(key) if !key.is_empty() => {
                // New non-empty key - replace
                existing.api_key = Some(key.clone());
            }
            Some(_) => {
                // Empty string - clear
                existing.api_key = None;
            }
            None => {
                // null - keep existing
            }
        }
        
        existing.updated_at = Utc::now().to_rfc3339();
    }
    
    merge_profile(&executor, &mut existing.executor);
    merge_profile(&evaluator, &mut existing.evaluator);
    merge_profile(&generator_override, &mut existing.generator_override);
    
    // Validation: if enabled, must have required fields
    if existing.executor.enabled && (existing.executor.base_url.is_empty() || existing.executor.model.is_empty() || existing.executor.api_key.is_none()) {
        return Err(CommandError {
            code: "PROFILES_VALIDATION_FAILED".into(),
            message: "执行AI配置不完整".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    if existing.evaluator.enabled && (existing.evaluator.base_url.is_empty() || existing.evaluator.model.is_empty() || existing.evaluator.api_key.is_none()) {
        return Err(CommandError {
            code: "PROFILES_VALIDATION_FAILED".into(),
            message: "测评AI配置不完整".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    if existing.generator_override.enabled && (existing.generator_override.base_url.is_empty() || existing.generator_override.model.is_empty() || existing.generator_override.api_key.is_none()) {
        return Err(CommandError {
            code: "PROFILES_VALIDATION_FAILED".into(),
            message: "出题AI配置不完整".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    
    // TODO: Handle encryption mode - for now just plain text
    // If api_key_storage_mode is "encrypted", we should encrypt but for v1 we'll just use plain
    // Add warning if encryption requested but not supported
    let mut warnings = Vec::new();
    for (profile_name, profile) in [("executor", &existing.executor), ("evaluator", &existing.evaluator), ("generator_override", &existing.generator_override)] {
        if profile.api_key_storage_mode == "encrypted" {
            warnings.push(CommandWarning {
                code: "API_KEY_ENCRYPTION_FAILED".into(),
                message: format!("{}: 加密模式暂不支持，已回退为明文存储", profile_name),
                severity: Severity::Warning,
                recoverable: true,
                action: Action::None,
            });
        }
    }
    
    write_profiles(&existing)?;
    
    // Return summary similar to get_saved_profiles_summary
    let summary = get_saved_profiles_summary()?;
    
    Ok(CommandSuccess {
        ok: true,
        data: summary.data,
        warnings,
    })
}

#[tauri::command]
fn clear_saved_profiles() -> Result<CommandSuccess<()>, CommandError> {
    let data_dir = get_app_data_dir()?;
    let profiles_path = data_dir.join("profiles.json");
    
    if profiles_path.exists() {
        std::fs::remove_file(&profiles_path).map_err(|e| CommandError {
            code: "PROFILES_WRITE_FAILED".into(),
            message: format!("无法删除配置文件: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    }
    
    Ok(CommandSuccess {
        ok: true,
        data: (),
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn reset_settings_to_default() -> Result<CommandSuccess<SettingsSummary>, CommandError> {
    let default_settings = default_settings();
    write_settings(&default_settings)?;
    
    Ok(CommandSuccess {
        ok: true,
        data: SettingsSummary {
            theme_color: default_settings.ui.theme_color,
            font_size: default_settings.ui.font_size,
            show_launch_notice: default_settings.privacy.show_launch_notice,
            default_advanced_evaluation_context: default_settings.assessment.default_advanced_evaluation_context,
            default_markdown_directory: default_settings.export.default_markdown_directory,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn save_settings(
    theme_color: String,
    font_size: String,
    show_launch_notice: bool,
    default_advanced_evaluation_context: bool,
    default_markdown_directory: Option<String>,
) -> Result<CommandSuccess<SettingsSummary>, CommandError> {
    // validation
    let allowed_themes = ["teal", "amber", "slate"];
    if !allowed_themes.contains(&theme_color.as_str()) {
        return Err(CommandError {
            code: "SETTINGS_VALIDATION_FAILED".into(),
            message: "主题颜色无效".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    let allowed_font_sizes = ["small", "medium", "large"];
    if !allowed_font_sizes.contains(&font_size.as_str()) {
        return Err(CommandError {
            code: "SETTINGS_VALIDATION_FAILED".into(),
            message: "字体大小无效".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    
    let mut settings = read_settings()?;
    settings.ui.theme_color = theme_color;
    settings.ui.font_size = font_size;
    settings.privacy.show_launch_notice = show_launch_notice;
    settings.assessment.default_advanced_evaluation_context = default_advanced_evaluation_context;
    settings.export.default_markdown_directory = default_markdown_directory;
    settings.updated_at = Utc::now().to_rfc3339();
    
    write_settings(&settings)?;
    
    Ok(CommandSuccess {
        ok: true,
        data: SettingsSummary {
            theme_color: settings.ui.theme_color,
            font_size: settings.ui.font_size,
            show_launch_notice: settings.privacy.show_launch_notice,
            default_advanced_evaluation_context: settings.assessment.default_advanced_evaluation_context,
            default_markdown_directory: settings.export.default_markdown_directory,
        },
        warnings: Vec::new(),
    })
}

fn get_app_data_dir() -> Result<std::path::PathBuf, CommandError> {
    let proj_dirs = ProjectDirs::from("", "", "PromptIQTest")
        .ok_or_else(|| CommandError {
            code: "SETTINGS_WRITE_FAILED".into(),
            message: "无法确定应用数据目录".into(),
            severity: Severity::Error,
            recoverable: false,
            action: Action::CloseDialog,
        })?;
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir).map_err(|e| CommandError {
        code: "SETTINGS_WRITE_FAILED".into(),
        message: format!("无法创建应用数据目录: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    Ok(data_dir.to_path_buf())
}

fn default_settings() -> Settings {
    Settings {
        schema_version: "1.0.0".into(),
        ui: SettingsUi {
            theme_color: "teal".into(),
            font_size: "medium".into(),
        },
        privacy: SettingsPrivacy {
            show_launch_notice: true,
        },
        assessment: SettingsAssessment {
            default_advanced_evaluation_context: false,
        },
        export: SettingsExport {
            default_markdown_directory: None,
        },
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}

fn read_settings() -> Result<Settings, CommandError> {
    let data_dir = get_app_data_dir()?;
    let settings_path = data_dir.join("settings.json");
    if !settings_path.exists() {
        return Ok(default_settings());
    }
    let content = std::fs::read_to_string(&settings_path).map_err(|e| CommandError {
        code: "SETTINGS_READ_FAILED".into(),
        message: format!("无法读取设置文件: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::ReloadCurrentPage,
    })?;
    serde_json::from_str(&content).map_err(|e| CommandError {
        code: "SETTINGS_VALIDATION_FAILED".into(),
        message: format!("设置文件格式无效: {}", e),
        severity: Severity::Warning,
        recoverable: true,
        action: Action::None,
    })
}

fn write_settings(settings: &Settings) -> Result<(), CommandError> {
    let data_dir = get_app_data_dir()?;
    let settings_path = data_dir.join("settings.json");
    let content = serde_json::to_string_pretty(settings).map_err(|e| CommandError {
        code: "SETTINGS_WRITE_FAILED".into(),
        message: format!("无法序列化设置: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    std::fs::write(&settings_path, content).map_err(|e| CommandError {
        code: "SETTINGS_WRITE_FAILED".into(),
        message: format!("无法写入设置文件: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })
}

fn default_profile() -> Profile {
    Profile {
        enabled: false,
        base_url: "".into(),
        model: "".into(),
        api_key_storage_mode: "plain".into(),
        api_key: None,
        temperature: 0.7,
        max_tokens: None,
        top_p: 1.0,
        timeout_ms: 60000,
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}

fn read_profiles() -> Result<Profiles, CommandError> {
    let data_dir = get_app_data_dir()?;
    let profiles_path = data_dir.join("profiles.json");
    if !profiles_path.exists() {
        return Ok(Profiles {
            schema_version: "1.0.0".into(),
            executor: default_profile(),
            evaluator: default_profile(),
            generator_override: default_profile(),
        });
    }
    let content = std::fs::read_to_string(&profiles_path).map_err(|e| CommandError {
        code: "PROFILES_READ_FAILED".into(),
        message: format!("无法读取配置文件: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::ReloadCurrentPage,
    })?;
    serde_json::from_str(&content).map_err(|e| CommandError {
        code: "PROFILES_VALIDATION_FAILED".into(),
        message: format!("配置文件格式无效: {}", e),
        severity: Severity::Warning,
        recoverable: true,
        action: Action::None,
    })
}

fn write_profiles(profiles: &Profiles) -> Result<(), CommandError> {
    let data_dir = get_app_data_dir()?;
    let profiles_path = data_dir.join("profiles.json");
    let content = serde_json::to_string_pretty(profiles).map_err(|e| CommandError {
        code: "PROFILES_WRITE_FAILED".into(),
        message: format!("无法序列化配置: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    std::fs::write(&profiles_path, content).map_err(|e| CommandError {
        code: "PROFILES_WRITE_FAILED".into(),
        message: format!("无法写入配置文件: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })
}

fn get_db_connection() -> Result<Connection, CommandError> {
    let data_dir = get_app_data_dir()?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(&db_path).map_err(|e| CommandError {
        code: "DB_OPEN_FAILED".into(),
        message: format!("无法打开数据库: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", []).map_err(|e| CommandError {
        code: "DB_OPEN_FAILED".into(),
        message: format!("无法启用外键约束: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    Ok(conn)
}

fn init_database() -> Result<(), CommandError> {
    let conn = get_db_connection()?;
    
    // Create migrations table to track applied migrations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            applied_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建migrations表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create assessment_sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assessment_sessions (
            id TEXT PRIMARY KEY,
            scope TEXT NOT NULL CHECK (scope IN ('fun', 'comprehensive')),
            interaction_mode TEXT NOT NULL CHECK (interaction_mode IN ('static', 'dynamic')),
            question_source TEXT NOT NULL CHECK (question_source IN ('builtin', 'ai_generated')),
            static_strategy TEXT CHECK (static_strategy IN ('prompt_only', 'execute_then_evaluate')),
            dynamic_end_strategy TEXT CHECK (dynamic_end_strategy IN ('fixed_rounds', 'manual_with_evaluator_suggestion')),
            fixed_round_limit INTEGER,
            task_type TEXT,
            generation_constraints TEXT,
            advanced_eval_context INTEGER NOT NULL DEFAULT 0,
            executor_model_snapshot_json TEXT,
            evaluator_model_snapshot_json TEXT NOT NULL,
            generator_model_snapshot_json TEXT,
            total_score REAL,
            confidence_level TEXT,
            confidence_note TEXT,
            question_count INTEGER NOT NULL DEFAULT 0,
            effective_round_count INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL CHECK (status IN ('draft', 'running', 'completed', 'failed', 'abandoned')),
            rubric_version TEXT NOT NULL,
            prompt_bundle_version TEXT NOT NULL,
            question_bundle_version TEXT,
            created_at TEXT NOT NULL,
            completed_at TEXT,
            updated_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建assessment_sessions表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create assessment_tasks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assessment_tasks (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES assessment_sessions(id) ON DELETE CASCADE,
            sequence_no INTEGER NOT NULL,
            source TEXT NOT NULL CHECK (source IN ('builtin', 'ai_generated')),
            task_type TEXT NOT NULL,
            title TEXT NOT NULL,
            visible_task TEXT NOT NULL,
            visible_requirements_json TEXT NOT NULL,
            raw_question_json TEXT NOT NULL,
            hidden_reference_json TEXT NOT NULL,
            dimension_focus_json TEXT NOT NULL,
            user_prompt TEXT,
            executor_output TEXT,
            prompt_score REAL,
            execution_score REAL,
            final_score REAL,
            created_at TEXT NOT NULL,
            completed_at TEXT
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建assessment_tasks表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create chat_messages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_messages (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES assessment_sessions(id) ON DELETE CASCADE,
            task_id TEXT REFERENCES assessment_tasks(id) ON DELETE CASCADE,
            round_no INTEGER NOT NULL,
            message_index INTEGER NOT NULL,
            role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system_internal')),
            actor_kind TEXT NOT NULL CHECK (actor_kind IN ('user', 'executor', 'evaluator', 'app')),
            content TEXT NOT NULL,
            model_name TEXT,
            created_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建chat_messages表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create evaluation_snapshots table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS evaluation_snapshots (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES assessment_sessions(id) ON DELETE CASCADE,
            task_id TEXT REFERENCES assessment_tasks(id) ON DELETE CASCADE,
            round_no INTEGER,
            snapshot_type TEXT NOT NULL CHECK (snapshot_type IN ('static_result', 'dynamic_live', 'dynamic_final')),
            current_total_score REAL NOT NULL,
            core_scores_json TEXT NOT NULL,
            short_comment TEXT,
            suggest_finish INTEGER NOT NULL DEFAULT 0,
            trend TEXT,
            raw_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建evaluation_snapshots表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create final_reports table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS final_reports (
            session_id TEXT PRIMARY KEY REFERENCES assessment_sessions(id) ON DELETE CASCADE,
            total_score REAL NOT NULL,
            confidence_level TEXT NOT NULL,
            confidence_note TEXT NOT NULL,
            rubric_score REAL NOT NULL,
            holistic_score REAL NOT NULL,
            task_completion_score REAL,
            core_scores_json TEXT NOT NULL,
            subdimension_scores_json TEXT NOT NULL,
            strengths_json TEXT NOT NULL,
            issues_json TEXT NOT NULL,
            suggestions_json TEXT NOT NULL,
            evidence_json TEXT NOT NULL,
            evaluator_summary TEXT NOT NULL,
            markdown_cache TEXT,
            raw_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建final_reports表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create markdown_exports table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS markdown_exports (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL REFERENCES assessment_sessions(id) ON DELETE CASCADE,
            export_path TEXT NOT NULL,
            export_format TEXT NOT NULL CHECK (export_format IN ('markdown')),
            created_at TEXT NOT NULL
        )",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建markdown_exports表: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    // Create indexes for better query performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sessions_completed_at ON assessment_sessions(completed_at DESC)",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建索引idx_sessions_completed_at: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_session_id ON assessment_tasks(session_id, sequence_no)",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建索引idx_tasks_session_id: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_session_round ON chat_messages(session_id, round_no, message_index)",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建索引idx_messages_session_round: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_snapshots_session_round ON evaluation_snapshots(session_id, round_no, created_at)",
        []
    ).map_err(|e| CommandError {
        code: "DB_WRITE_FAILED".into(),
        message: format!("无法创建索引idx_snapshots_session_round: {}", e),
        severity: Severity::Error,
        recoverable: false,
        action: Action::CloseDialog,
    })?;
    
    Ok(())
}

#[tauri::command]
fn get_history_sessions(limit: u32) -> Result<CommandSuccess<Vec<db::AssessmentSession>>, CommandError> {
    let conn = get_db_connection()?;
    let sessions = db::AssessmentSession::list_recent(&conn, limit as i32)?;
    
    Ok(CommandSuccess {
        ok: true,
        data: sessions,
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn delete_history_session(session_id: String) -> Result<CommandSuccess<()>, CommandError> {
    let conn = get_db_connection()?;
    db::AssessmentSession::delete(&conn, &session_id)?;
    
    Ok(CommandSuccess {
        ok: true,
        data: (),
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn get_session_details(session_id: String) -> Result<CommandSuccess<db::AssessmentSession>, CommandError> {
    let conn = get_db_connection()?;
    let session = db::AssessmentSession::get_by_id(&conn, &session_id)?
        .ok_or_else(|| CommandError {
            code: "SESSION_NOT_FOUND".into(),
            message: "会话不存在".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        })?;
    
    Ok(CommandSuccess {
        ok: true,
        data: session,
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn clear_chat_messages() -> Result<CommandSuccess<ClearChatMessagesResult>, CommandError> {
    let conn = get_db_connection()?;
    
    // First count affected sessions and messages before deletion
    let affected_session_count: u32 = conn.query_row(
        "SELECT COUNT(DISTINCT session_id) FROM chat_messages",
        [],
        |row| row.get(0)
    ).map_err(|e| CommandError {
        code: "DB_QUERY_FAILED".into(),
        message: format!("无法统计受影响会话: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    
    let deleted_message_count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM chat_messages",
        [],
        |row| row.get(0)
    ).map_err(|e| CommandError {
        code: "DB_QUERY_FAILED".into(),
        message: format!("无法统计聊天消息数量: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    
    conn.execute("DELETE FROM chat_messages", [])
        .map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法删除聊天消息: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    Ok(CommandSuccess {
        ok: true,
        data: ClearChatMessagesResult {
            deleted_message_count,
            affected_session_count,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn clear_question_cache() -> Result<CommandSuccess<ClearQuestionCacheResult>, CommandError> {
    let conn = get_db_connection()?;
    
    // Count incomplete sessions before deletion
    let deleted_session_count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM assessment_sessions WHERE status != 'completed'",
        [],
        |row| row.get(0)
    ).map_err(|e| CommandError {
        code: "DB_QUERY_FAILED".into(),
        message: format!("无法统计未完成会话: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    
    // Count question cache entries (tasks from incomplete sessions)
    let deleted_question_cache_entry_count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM assessment_tasks WHERE session_id IN (SELECT id FROM assessment_sessions WHERE status != 'completed')",
        [],
        |row| row.get(0)
    ).map_err(|e| CommandError {
        code: "DB_QUERY_FAILED".into(),
        message: format!("无法统计题目缓存条目: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    
    // Delete incomplete sessions (cascade will delete related data)
    conn.execute("DELETE FROM assessment_sessions WHERE status != 'completed'", [])
        .map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法删除未完成会话: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    Ok(CommandSuccess {
        ok: true,
        data: ClearQuestionCacheResult {
            deleted_session_count,
            deleted_question_cache_entry_count,
        },
        warnings: Vec::new(),
    })
}

// ==================== WP-06 Assessment Session Commands ====================

#[tauri::command]
fn start_assessment_session(
    request: AssessmentStartRequest,
) -> Result<CommandSuccess<StartAssessmentSuccess>, CommandError> {
    let profiles = read_profiles()?;
    let conn = get_db_connection()?;
    let now = Utc::now().to_rfc3339();
    
    // 1. Validate mode combination
    if request.interaction_mode != "static" {
        return Err(CommandError {
            code: "ASSESSMENT_CONFIG_INVALID".into(),
            message: "当前仅支持静态测评模式".into(),
            severity: Severity::Warning,
            recoverable: false,
            action: Action::None,
        });
    }
    
    let static_strategy = request.static_strategy.as_deref().unwrap_or("prompt_only");
    if static_strategy != "prompt_only" && static_strategy != "execute_then_evaluate" {
        return Err(CommandError {
            code: "ASSESSMENT_CONFIG_INVALID".into(),
            message: "静态策略必须为 prompt_only 或 execute_then_evaluate".into(),
            severity: Severity::Warning,
            recoverable: false,
            action: Action::None,
        });
    }
    
    // 2. Validate required configurations
    if !profiles.evaluator.enabled || profiles.evaluator.api_key.is_none() {
        return Err(CommandError {
            code: "CONFIG_MISSING_EVALUATOR".into(),
            message: "测评AI配置缺失，请先在设置页配置".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::GoToSettings,
        });
    }
    
    if static_strategy == "execute_then_evaluate" {
        if !profiles.executor.enabled || profiles.executor.api_key.is_none() {
            return Err(CommandError {
                code: "CONFIG_MISSING_EXECUTOR".into(),
                message: "执行AI配置缺失，请先在设置页配置".into(),
                severity: Severity::Warning,
                recoverable: true,
                action: Action::GoToSettings,
            });
        }
    }
    
    // 3. Create model snapshots
    let evaluator_snapshot = ModelProfileSummary {
        enabled: profiles.evaluator.enabled,
        base_url: profiles.evaluator.base_url.clone(),
        model: profiles.evaluator.model.clone(),
        has_api_key: profiles.evaluator.api_key.is_some(),
        api_key_storage_mode: profiles.evaluator.api_key_storage_mode.clone(),
        temperature: profiles.evaluator.temperature,
        max_tokens: profiles.evaluator.max_tokens,
        top_p: profiles.evaluator.top_p,
        timeout_ms: profiles.evaluator.timeout_ms,
        updated_at: profiles.evaluator.updated_at.clone(),
    };
    
    let executor_snapshot = if profiles.executor.enabled {
        Some(ModelProfileSummary {
            enabled: profiles.executor.enabled,
            base_url: profiles.executor.base_url.clone(),
            model: profiles.executor.model.clone(),
            has_api_key: profiles.executor.api_key.is_some(),
            api_key_storage_mode: profiles.executor.api_key_storage_mode.clone(),
            temperature: profiles.executor.temperature,
            max_tokens: profiles.executor.max_tokens,
            top_p: profiles.executor.top_p,
            timeout_ms: profiles.executor.timeout_ms,
            updated_at: profiles.executor.updated_at.clone(),
        })
    } else {
        None
    };
    
    let generator_snapshot = if profiles.generator_override.enabled {
        Some(ModelProfileSummary {
            enabled: profiles.generator_override.enabled,
            base_url: profiles.generator_override.base_url.clone(),
            model: profiles.generator_override.model.clone(),
            has_api_key: profiles.generator_override.api_key.is_some(),
            api_key_storage_mode: profiles.generator_override.api_key_storage_mode.clone(),
            temperature: profiles.generator_override.temperature,
            max_tokens: profiles.generator_override.max_tokens,
            top_p: profiles.generator_override.top_p,
            timeout_ms: profiles.generator_override.timeout_ms,
            updated_at: profiles.generator_override.updated_at.clone(),
        })
    } else {
        None
    };
    
    // 4. Draw questions from built-in bank
    let all_questions = question_bank::load_builtin_questions()
        .map_err(|e| CommandError {
            code: "QUESTION_BANK_LOAD_FAILED".into(),
            message: format!("无法加载题库: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    let scope = &request.scope;
    let filtered: Vec<question_bank::QuestionSchema> = all_questions
        .into_iter()
        .filter(|q| q.assessment_mode == "static" && q.scope_fit.contains(&scope.to_string()))
        .collect();
    
    if filtered.is_empty() {
        return Err(CommandError {
            code: "QUESTION_BANK_EMPTY".into(),
            message: "没有符合条件的题目".into(),
            severity: Severity::Warning,
            recoverable: false,
            action: Action::None,
        });
    }
    
    let question_count = if request.scope == "fun" {
        std::cmp::min(filtered.len(), 3) as i32
    } else {
        std::cmp::min(filtered.len(), 5) as i32
    };
    
    let selected_questions: Vec<&question_bank::QuestionSchema> = filtered.iter().take(question_count as usize).collect();
    
    // 5. Create assessment session
    let session_id = uuid::Uuid::new_v4().to_string();
    
    let session = db::AssessmentSession {
        id: session_id.clone(),
        scope: request.scope.clone(),
        interaction_mode: request.interaction_mode.clone(),
        question_source: request.question_source.clone(),
        static_strategy: Some(static_strategy.to_string()),
        dynamic_end_strategy: None,
        fixed_round_limit: None,
        task_type: request.task_type.clone(),
        generation_constraints: None,
        advanced_eval_context: if request.advanced_evaluation_context { 1 } else { 0 },
        executor_model_snapshot_json: executor_snapshot.as_ref().map(|s| serde_json::to_string(s).unwrap_or_default()),
        evaluator_model_snapshot_json: serde_json::to_string(&evaluator_snapshot).unwrap_or_default(),
        generator_model_snapshot_json: generator_snapshot.as_ref().map(|s| serde_json::to_string(s).unwrap_or_default()),
        total_score: None,
        confidence_level: None,
        confidence_note: None,
        question_count,
        effective_round_count: 0,
        status: "running".to_string(),
        rubric_version: "rubric-v1".to_string(),
        prompt_bundle_version: "eval-static-prompt-only-v1".to_string(),
        question_bundle_version: Some("builtin-v1".to_string()),
        created_at: now.clone(),
        completed_at: None,
        updated_at: now.clone(),
    };
    
    session.insert(&conn)?;
    
    // 6. Create assessment tasks from selected questions
    for (idx, question) in selected_questions.iter().enumerate() {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = db::AssessmentTask {
            id: task_id,
            session_id: session_id.clone(),
            sequence_no: (idx + 1) as i32,
            source: "builtin".to_string(),
            task_type: question.task_type.clone(),
            title: question.title.clone(),
            visible_task: question.visible_prompt.task.clone(),
            visible_requirements_json: serde_json::to_string(&question.visible_prompt.requirements).unwrap_or_default(),
            raw_question_json: serde_json::to_string(question).unwrap_or_default(),
            hidden_reference_json: serde_json::to_string(&question.hidden_reference).unwrap_or_default(),
            dimension_focus_json: serde_json::to_string(&question.hidden_reference.dimension_focus).unwrap_or_default(),
            user_prompt: None,
            executor_output: None,
            prompt_score: None,
            execution_score: None,
            final_score: None,
            created_at: now.clone(),
            completed_at: None,
        };
        task.insert(&conn)?;
    }
    
    // 7. Return entry data
    let session_summary = SessionSummary {
        session_id,
        scope: request.scope.clone(),
        interaction_mode: request.interaction_mode.clone(),
        question_source: request.question_source.clone(),
        status: "running".to_string(),
        question_count,
        effective_round_count: 0,
        advanced_evaluation_context: request.advanced_evaluation_context,
        static_strategy: Some(static_strategy.to_string()),
        executor_model_snapshot: executor_snapshot,
        evaluator_model_snapshot: evaluator_snapshot,
        generator_model_snapshot: generator_snapshot,
        created_at: now,
    };
    
    Ok(CommandSuccess {
        ok: true,
        data: StartAssessmentSuccess {
            entry_mode: "static".to_string(),
            session: session_summary,
            initial_task_ready: true,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn load_next_static_task(
    session_id: String,
) -> Result<CommandSuccess<StaticTaskPresentation>, CommandError> {
    let conn = get_db_connection()?;
    
    // Validate session exists and is running
    let session = db::AssessmentSession::get_by_id(&conn, &session_id)?
        .ok_or_else(|| CommandError {
            code: "SESSION_NOT_FOUND".into(),
            message: "会话不存在".into(),
            severity: Severity::Error,
            recoverable: false,
            action: Action::BackToHome,
        })?;
    
    if session.status != "running" {
        return Err(CommandError {
            code: "ASSESSMENT_NOT_ACTIVE".into(),
            message: "会话已结束或已放弃".into(),
            severity: Severity::Warning,
            recoverable: false,
            action: Action::BackToHome,
        });
    }
    
    if session.interaction_mode != "static" {
        return Err(CommandError {
            code: "SESSION_MODE_MISMATCH".into(),
            message: "当前会话不是静态测评".into(),
            severity: Severity::Error,
            recoverable: false,
            action: Action::BackToHome,
        });
    }
    
    // Find next unsubmitted task
    let next_task = db::AssessmentTask::get_next_unsubmitted(&conn, &session_id)?
        .ok_or_else(|| CommandError {
            code: "QUESTION_NOT_FOUND".into(),
            message: "没有未提交的题目".into(),
            severity: Severity::Error,
            recoverable: true,
            action: Action::ReloadCurrentPage,
        })?;
    
    let all_tasks = db::AssessmentTask::get_by_session_id(&conn, &session_id)?;
    let completed_count = db::AssessmentTask::count_completed_by_session(&conn, &session_id)?;
    let total_count = all_tasks.len() as i32;
    
    let requirements: Vec<String> = serde_json::from_str(&next_task.visible_requirements_json).unwrap_or_default();
    
    let presentation = StaticTaskPresentation {
        state: "ready".to_string(),
        task: Some(StaticTaskData {
            task_id: next_task.id,
            sequence_no: next_task.sequence_no,
            title: next_task.title,
            task: next_task.visible_task,
            requirements,
        }),
        progress: TaskProgress {
            current_index: next_task.sequence_no,
            total_count,
            completed_count,
            remaining_count: total_count - completed_count,
        },
    };
    
    Ok(CommandSuccess {
        ok: true,
        data: presentation,
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn submit_static_prompt(
    session_id: String,
    task_id: String,
    user_prompt: String,
) -> Result<CommandSuccess<SubmitStaticPromptSuccess>, CommandError> {
    let profiles = read_profiles()?;
    let conn = get_db_connection()?;
    
    // Validate session exists and is running
    let session = db::AssessmentSession::get_by_id(&conn, &session_id)?
        .ok_or_else(|| CommandError {
            code: "SESSION_NOT_FOUND".into(),
            message: "会话不存在".into(),
            severity: Severity::Error,
            recoverable: false,
            action: Action::BackToHome,
        })?;
    
    if session.status != "running" {
        return Err(CommandError {
            code: "ASSESSMENT_NOT_ACTIVE".into(),
            message: "会话已结束或已放弃".into(),
            severity: Severity::Warning,
            recoverable: false,
            action: Action::BackToHome,
        });
    }
    
    // Validate task exists and is not yet submitted
    let task = db::AssessmentTask::get_by_id(&conn, &task_id)?
        .ok_or_else(|| CommandError {
            code: "QUESTION_NOT_FOUND".into(),
            message: "题目不存在".into(),
            severity: Severity::Error,
            recoverable: true,
            action: Action::ReloadCurrentPage,
        })?;
    
    if task.user_prompt.is_some() {
        return Err(CommandError {
            code: "TASK_ALREADY_SUBMITTED".into(),
            message: "本题已提交过".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::ReloadCurrentPage,
        });
    }
    
    // Ensure evaluator profile is configured
    if !profiles.evaluator.enabled || profiles.evaluator.api_key.is_none() {
        return Err(CommandError {
            code: "CONFIG_MISSING_EVALUATOR".into(),
            message: "测评AI配置缺失，请在设置页配置".into(),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::GoToSettings,
        });
    }
    
    // Call evaluator AI to score the prompt
    let evaluator_prompt = build_evaluator_prompt(&task, &user_prompt, session.advanced_eval_context != 0);
    
    let advanced_eval = session.advanced_eval_context != 0;
    let evaluator_messages = vec![
        ai_client::ChatMessage {
            role: "system".to_string(),
            content: "You are the evaluator model inside Prompt IQ Test. Evaluate the human user's ability to drive LLM/Agent systems. Score only from provided evidence. Return only valid JSON. All scores must be integers from 0 to 100. Be strict, stable, and reproducible.".to_string(),
        },
        ai_client::ChatMessage {
            role: "user".to_string(),
            content: evaluator_prompt,
        },
    ];
    
    // Create AI client from evaluator profile
    let client = ai_client::client_from_profile(&profiles.evaluator)
        .map_err(|e| CommandError {
            code: "MODEL_AUTH_FAILED".into(),
            message: format!("无法创建AI客户端: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::GoToSettings,
        })?;
    
    // Execute AI call synchronously using a tokio runtime
    let response = tokio::runtime::Runtime::new()
        .map_err(|e| CommandError {
            code: "MODEL_REQUEST_FAILED".into(),
            message: format!("无法创建异步运行时: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?
        .block_on(async {
            client.chat_completion_json(
                evaluator_messages,
                Some(0.2), // Low temperature for stable evaluation
                Some(4096), // Max tokens
            ).await
        });
    
    let completion = response.map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("401") || error_msg.contains("Unauthorized") {
            CommandError {
                code: "MODEL_AUTH_FAILED".into(),
                message: "API认证失败，请检查API Key".into(),
                severity: Severity::Error,
                recoverable: true,
                action: Action::GoToSettings,
            }
        } else if error_msg.contains("timeout") {
            CommandError {
                code: "MODEL_TIMEOUT".into(),
                message: "AI请求超时".into(),
                severity: Severity::Error,
                recoverable: true,
                action: Action::RetrySameAction,
            }
        } else if error_msg.contains("model") || error_msg.contains("Model") {
            CommandError {
                code: "MODEL_NOT_FOUND".into(),
                message: format!("模型调用失败: {}", error_msg),
                severity: Severity::Error,
                recoverable: true,
                action: Action::GoToSettings,
            }
        } else {
            CommandError {
                code: "MODEL_REQUEST_FAILED".into(),
                message: format!("AI请求失败: {}", error_msg),
                severity: Severity::Error,
                recoverable: true,
                action: Action::RetrySameAction,
            }
        }
    })?;
    
    // Parse response content
    let response_content = completion.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| CommandError {
            code: "MODEL_EMPTY_RESPONSE".into(),
            message: "AI返回空响应".into(),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    // Parse JSON response
    let eval_json = parse_evaluation_response(&response_content).map_err(|e| CommandError {
        code: "MODEL_INVALID_JSON".into(),
        message: format!("无法解析评分JSON: {}", e),
        severity: Severity::Error,
        recoverable: true,
        action: Action::RetrySameAction,
    })?;
    
    let prompt_score = eval_json.get("rubricScore").and_then(|v| v.as_f64());
    let execution_score = eval_json.get("executionOutcomeScore").and_then(|v| v.as_f64());
    let final_score = calculate_final_score_from_evaluation(&eval_json).ok();
    
    // Update task scores
    db::AssessmentTask::update_score(
        &conn,
        &task_id,
        &user_prompt,
        prompt_score,
        execution_score,
        final_score,
        None,
    )?;
    
    // Check if all tasks completed
    let total_count = session.question_count;
    let completed_count = db::AssessmentTask::count_completed_by_session(&conn, &session_id)?;
    
    if completed_count >= total_count {
        // All tasks completed - generate final report
        let core_scores = extract_core_scores(&eval_json).unwrap_or(CoreScoresSummary {
            clarity_output: 0.0,
            structuring: 0.0,
            context_constraints: 0.0,
            gap_risk: 0.0,
            iteration_acceptance: 0.0,
        });
        
        let final_score_value = final_score.unwrap_or(0.0);
        let strengths = extract_string_array(&eval_json, "strengths");
        let issues = extract_string_array(&eval_json, "issues");
        let suggestions = extract_string_array(&eval_json, "suggestions");
        let summary = eval_json.get("summary").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let confidence = extract_confidence(&eval_json);
        
        // Update session status to completed
        db::AssessmentSession::update_status(
            &conn,
            &session_id,
            "completed",
            Some(final_score_value),
            Some(&confidence.level),
            Some(&confidence.note),
        )?;
        
        let final_report = FinalReportSummary {
            session_id: session_id.clone(),
            scope: session.scope.clone(),
            interaction_mode: session.interaction_mode.clone(),
            static_strategy: session.static_strategy.clone(),
            total_score: final_score_value,
            core_scores,
            strengths,
            issues,
            suggestions,
            summary,
            confidence,
            question_count: total_count,
            effective_round_count: 0,
        };
        
        return Ok(CommandSuccess {
            ok: true,
            data: SubmitStaticPromptSuccess {
                next_action: "show_result".to_string(),
                scored_task: Some(ScoredTaskSummary {
                    task_id: task_id.clone(),
                    final_score: final_score_value,
                    executor_output_preview: None,
                }),
                progress: Some(TaskProgress {
                    current_index: completed_count,
                    total_count,
                    completed_count,
                    remaining_count: 0,
                }),
                final_report: Some(final_report),
            },
            warnings: Vec::new(),
        });
    }
    
    // Not all completed - return continue
    let remaining_count = total_count - completed_count;
    
    Ok(CommandSuccess {
        ok: true,
        data: SubmitStaticPromptSuccess {
            next_action: "continue".to_string(),
            scored_task: Some(ScoredTaskSummary {
                task_id: task_id.clone(),
                final_score: final_score.unwrap_or(0.0),
                executor_output_preview: None,
            }),
            progress: Some(TaskProgress {
                current_index: completed_count,
                total_count,
                completed_count,
                remaining_count,
            }),
            final_report: None,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn get_question_bank_summary() -> Result<CommandSuccess<QuestionBankSummary>, CommandError> {
    let questions = question_bank::load_builtin_questions()
        .map_err(|e| CommandError {
            code: "QUESTION_BANK_LOAD_FAILED".into(),
            message: format!("无法加载题库: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    let total_questions = questions.len() as u32;
    let static_questions = questions.iter().filter(|q| q.assessment_mode == "static").count() as u32;
    let dynamic_questions = questions.iter().filter(|q| q.assessment_mode == "dynamic").count() as u32;
    
    let mut by_task_type = std::collections::HashMap::new();
    for q in &questions {
        *by_task_type.entry(q.task_type.clone()).or_insert(0) += 1;
    }
    
    Ok(CommandSuccess {
        ok: true,
        data: QuestionBankSummary {
            total_questions,
            static_questions,
            dynamic_questions,
            by_task_type,
        },
        warnings: Vec::new(),
    })
}

#[tauri::command]
fn draw_questions(
    assessment_mode: String,
    scope: String,
    count: u32,
    task_type_filter: Option<String>,
) -> Result<CommandSuccess<Vec<QuestionSchema>>, CommandError> {
    let questions = question_bank::load_builtin_questions()
        .map_err(|e| CommandError {
            code: "QUESTION_BANK_LOAD_FAILED".into(),
            message: format!("无法加载题库: {}", e),
            severity: Severity::Error,
            recoverable: true,
            action: Action::RetrySameAction,
        })?;
    
    // Filter by assessment mode and scope
    let filtered: Vec<QuestionSchema> = questions
        .into_iter()
        .filter(|q| q.assessment_mode == assessment_mode && q.scope_fit.contains(&scope))
        .collect();
    
    // Further filter by task type if provided
    let filtered = if let Some(ref task_type) = task_type_filter {
        filtered.into_iter().filter(|q| q.task_type == *task_type).collect()
    } else {
        filtered
    };
    
    // Check if we have enough questions
    if filtered.len() < count as usize {
        return Err(CommandError {
            code: "INSUFFICIENT_QUESTIONS".into(),
            message: format!("符合条件的题目不足，需要 {} 题，但只有 {} 题", count, filtered.len()),
            severity: Severity::Warning,
            recoverable: true,
            action: Action::None,
        });
    }
    
    // Simple random selection (for now, just take first count)
    // TODO: Implement proper random sampling
    let selected = filtered.into_iter().take(count as usize).collect();
    
    Ok(CommandSuccess {
        ok: true,
        data: selected,
        warnings: Vec::new(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // Initialize database on startup
  if let Err(e) = init_database() {
    eprintln!("Failed to initialize database: {:?}", e);
    // Continue anyway - the app will show errors when database operations fail
  }
  
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_app_bootstrap,
      get_saved_profiles_summary,
      save_settings,
      save_profiles,
      clear_saved_profiles,
      reset_settings_to_default,
      get_history_sessions,
      delete_history_session,
      get_session_details,
      clear_chat_messages,
      clear_question_cache,
      get_question_bank_summary,
      draw_questions,
      start_assessment_session,
      load_next_static_task,
      submit_static_prompt
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
