use rusqlite::{Connection, params, OptionalExtension};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::CommandError;

// ==================== AssessmentSession DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentSession {
    pub id: String,
    pub scope: String, // 'fun' or 'comprehensive'
    pub interaction_mode: String, // 'static' or 'dynamic'
    pub question_source: String, // 'builtin' or 'ai_generated'
    pub static_strategy: Option<String>, // 'prompt_only' or 'execute_then_evaluate'
    pub dynamic_end_strategy: Option<String>, // 'fixed_rounds' or 'manual_with_evaluator_suggestion'
    pub fixed_round_limit: Option<i32>,
    pub task_type: Option<String>,
    pub generation_constraints: Option<String>,
    pub advanced_eval_context: i32, // 0 or 1
    pub executor_model_snapshot_json: Option<String>,
    pub evaluator_model_snapshot_json: String,
    pub generator_model_snapshot_json: Option<String>,
    pub total_score: Option<f64>,
    pub confidence_level: Option<String>,
    pub confidence_note: Option<String>,
    pub question_count: i32,
    pub effective_round_count: i32,
    pub status: String, // 'draft', 'running', 'completed', 'failed', 'abandoned'
    pub rubric_version: String,
    pub prompt_bundle_version: String,
    pub question_bundle_version: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

impl AssessmentSession {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO assessment_sessions (
                id, scope, interaction_mode, question_source, static_strategy, dynamic_end_strategy,
                fixed_round_limit, task_type, generation_constraints, advanced_eval_context,
                executor_model_snapshot_json, evaluator_model_snapshot_json, generator_model_snapshot_json,
                total_score, confidence_level, confidence_note, question_count, effective_round_count,
                status, rubric_version, prompt_bundle_version, question_bundle_version,
                created_at, completed_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)",
            params![
                &self.id, &self.scope, &self.interaction_mode, &self.question_source,
                &self.static_strategy, &self.dynamic_end_strategy, &self.fixed_round_limit,
                &self.task_type, &self.generation_constraints, &self.advanced_eval_context,
                &self.executor_model_snapshot_json, &self.evaluator_model_snapshot_json,
                &self.generator_model_snapshot_json, &self.total_score, &self.confidence_level,
                &self.confidence_note, self.question_count, self.effective_round_count,
                &self.status, &self.rubric_version, &self.prompt_bundle_version,
                &self.question_bundle_version, &self.created_at, &self.completed_at,
                &self.updated_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入assessment_sessions记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<AssessmentSession>, CommandError> {
        conn.query_row(
            "SELECT * FROM assessment_sessions WHERE id = ?1",
            params![id],
            |row| {
                Ok(AssessmentSession {
                    id: row.get(0)?,
                    scope: row.get(1)?,
                    interaction_mode: row.get(2)?,
                    question_source: row.get(3)?,
                    static_strategy: row.get(4)?,
                    dynamic_end_strategy: row.get(5)?,
                    fixed_round_limit: row.get(6)?,
                    task_type: row.get(7)?,
                    generation_constraints: row.get(8)?,
                    advanced_eval_context: row.get(9)?,
                    executor_model_snapshot_json: row.get(10)?,
                    evaluator_model_snapshot_json: row.get(11)?,
                    generator_model_snapshot_json: row.get(12)?,
                    total_score: row.get(13)?,
                    confidence_level: row.get(14)?,
                    confidence_note: row.get(15)?,
                    question_count: row.get(16)?,
                    effective_round_count: row.get(17)?,
                    status: row.get(18)?,
                    rubric_version: row.get(19)?,
                    prompt_bundle_version: row.get(20)?,
                    question_bundle_version: row.get(21)?,
                    created_at: row.get(22)?,
                    completed_at: row.get(23)?,
                    updated_at: row.get(24)?,
                })
            }
        ).optional().map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法读取assessment_sessions记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })
    }

    pub fn update(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "UPDATE assessment_sessions SET
                scope = ?2, interaction_mode = ?3, question_source = ?4, static_strategy = ?5,
                dynamic_end_strategy = ?6, fixed_round_limit = ?7, task_type = ?8,
                generation_constraints = ?9, advanced_eval_context = ?10,
                executor_model_snapshot_json = ?11, evaluator_model_snapshot_json = ?12,
                generator_model_snapshot_json = ?13, total_score = ?14, confidence_level = ?15,
                confidence_note = ?16, question_count = ?17, effective_round_count = ?18,
                status = ?19, rubric_version = ?20, prompt_bundle_version = ?21,
                question_bundle_version = ?22, completed_at = ?23, updated_at = ?24
            WHERE id = ?1",
            params![
                &self.id, &self.scope, &self.interaction_mode, &self.question_source,
                &self.static_strategy, &self.dynamic_end_strategy, &self.fixed_round_limit,
                &self.task_type, &self.generation_constraints, &self.advanced_eval_context,
                &self.executor_model_snapshot_json, &self.evaluator_model_snapshot_json,
                &self.generator_model_snapshot_json, &self.total_score, &self.confidence_level,
                &self.confidence_note, self.question_count, self.effective_round_count,
                &self.status, &self.rubric_version, &self.prompt_bundle_version,
                &self.question_bundle_version, &self.completed_at, &self.updated_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法更新assessment_sessions记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: &str) -> Result<(), CommandError> {
        conn.execute(
            "DELETE FROM assessment_sessions WHERE id = ?1",
            params![id],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法删除assessment_sessions记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn list_recent(conn: &Connection, limit: i32) -> Result<Vec<AssessmentSession>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM assessment_sessions ORDER BY created_at DESC LIMIT ?1"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let rows = stmt.query_map(params![limit], |row| {
            Ok(AssessmentSession {
                id: row.get(0)?,
                scope: row.get(1)?,
                interaction_mode: row.get(2)?,
                question_source: row.get(3)?,
                static_strategy: row.get(4)?,
                dynamic_end_strategy: row.get(5)?,
                fixed_round_limit: row.get(6)?,
                task_type: row.get(7)?,
                generation_constraints: row.get(8)?,
                advanced_eval_context: row.get(9)?,
                executor_model_snapshot_json: row.get(10)?,
                evaluator_model_snapshot_json: row.get(11)?,
                generator_model_snapshot_json: row.get(12)?,
                total_score: row.get(13)?,
                confidence_level: row.get(14)?,
                confidence_note: row.get(15)?,
                question_count: row.get(16)?,
                effective_round_count: row.get(17)?,
                status: row.get(18)?,
                rubric_version: row.get(19)?,
                prompt_bundle_version: row.get(20)?,
                question_bundle_version: row.get(21)?,
                created_at: row.get(22)?,
                completed_at: row.get(23)?,
                updated_at: row.get(24)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询assessment_sessions: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row.map_err(|e| CommandError {
                code: "DB_READ_FAILED".into(),
                message: format!("无法解析assessment_sessions记录: {}", e),
                severity: crate::Severity::Error,
                recoverable: false,
                action: crate::Action::CloseDialog,
            })?);
        }
        Ok(sessions)
    }
}

// ==================== AssessmentTask DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentTask {
    pub id: String,
    pub session_id: String,
    pub sequence_no: i32,
    pub source: String, // 'builtin' or 'ai_generated'
    pub task_type: String,
    pub title: String,
    pub visible_task: String,
    pub visible_requirements_json: String,
    pub raw_question_json: String,
    pub hidden_reference_json: String,
    pub dimension_focus_json: String,
    pub user_prompt: Option<String>,
    pub executor_output: Option<String>,
    pub prompt_score: Option<f64>,
    pub execution_score: Option<f64>,
    pub final_score: Option<f64>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

impl AssessmentTask {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO assessment_tasks (
                id, session_id, sequence_no, source, task_type, title, visible_task,
                visible_requirements_json, raw_question_json, hidden_reference_json,
                dimension_focus_json, user_prompt, executor_output, prompt_score,
                execution_score, final_score, created_at, completed_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                &self.id, &self.session_id, self.sequence_no, &self.source, &self.task_type,
                &self.title, &self.visible_task, &self.visible_requirements_json,
                &self.raw_question_json, &self.hidden_reference_json, &self.dimension_focus_json,
                &self.user_prompt, &self.executor_output, &self.prompt_score,
                &self.execution_score, &self.final_score, &self.created_at, &self.completed_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入assessment_tasks记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_by_session_id(conn: &Connection, session_id: &str) -> Result<Vec<AssessmentTask>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM assessment_tasks WHERE session_id = ?1 ORDER BY sequence_no"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(AssessmentTask {
                id: row.get(0)?,
                session_id: row.get(1)?,
                sequence_no: row.get(2)?,
                source: row.get(3)?,
                task_type: row.get(4)?,
                title: row.get(5)?,
                visible_task: row.get(6)?,
                visible_requirements_json: row.get(7)?,
                raw_question_json: row.get(8)?,
                hidden_reference_json: row.get(9)?,
                dimension_focus_json: row.get(10)?,
                user_prompt: row.get(11)?,
                executor_output: row.get(12)?,
                prompt_score: row.get(13)?,
                execution_score: row.get(14)?,
                final_score: row.get(15)?,
                created_at: row.get(16)?,
                completed_at: row.get(17)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询assessment_tasks: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row.map_err(|e| CommandError {
                code: "DB_READ_FAILED".into(),
                message: format!("无法解析assessment_tasks记录: {}", e),
                severity: crate::Severity::Error,
                recoverable: false,
                action: crate::Action::CloseDialog,
            })?);
        }
        Ok(tasks)
    }
}

// ==================== ChatMessage DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub task_id: Option<String>,
    pub round_no: i32,
    pub message_index: i32,
    pub role: String, // 'user', 'assistant', 'system_internal'
    pub actor_kind: String, // 'user', 'executor', 'evaluator', 'app'
    pub content: String,
    pub model_name: Option<String>,
    pub created_at: String,
}

impl ChatMessage {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO chat_messages (
                id, session_id, task_id, round_no, message_index, role, actor_kind,
                content, model_name, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &self.id, &self.session_id, &self.task_id, self.round_no, self.message_index,
                &self.role, &self.actor_kind, &self.content, &self.model_name, &self.created_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入chat_messages记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_by_session_id(conn: &Connection, session_id: &str) -> Result<Vec<ChatMessage>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM chat_messages WHERE session_id = ?1 ORDER BY round_no, message_index"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(ChatMessage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                task_id: row.get(2)?,
                round_no: row.get(3)?,
                message_index: row.get(4)?,
                role: row.get(5)?,
                actor_kind: row.get(6)?,
                content: row.get(7)?,
                model_name: row.get(8)?,
                created_at: row.get(9)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询chat_messages: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut messages = Vec::new();
        for row in rows {
            messages.push(row.map_err(|e| CommandError {
                code: "DB_READ_FAILED".into(),
                message: format!("无法解析chat_messages记录: {}", e),
                severity: crate::Severity::Error,
                recoverable: false,
                action: crate::Action::CloseDialog,
            })?);
        }
        Ok(messages)
    }
}

// ==================== EvaluationSnapshot DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationSnapshot {
    pub id: String,
    pub session_id: String,
    pub task_id: Option<String>,
    pub round_no: Option<i32>,
    pub snapshot_type: String, // 'static_result', 'dynamic_live', 'dynamic_final'
    pub current_total_score: f64,
    pub core_scores_json: String,
    pub short_comment: Option<String>,
    pub suggest_finish: i32, // 0 or 1
    pub trend: Option<String>,
    pub raw_json: String,
    pub created_at: String,
}

impl EvaluationSnapshot {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO evaluation_snapshots (
                id, session_id, task_id, round_no, snapshot_type, current_total_score,
                core_scores_json, short_comment, suggest_finish, trend, raw_json, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &self.id, &self.session_id, &self.task_id, &self.round_no, &self.snapshot_type,
                self.current_total_score, &self.core_scores_json, &self.short_comment,
                self.suggest_finish, &self.trend, &self.raw_json, &self.created_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入evaluation_snapshots记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }
}

// ==================== FinalReport DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalReport {
    pub session_id: String,
    pub total_score: f64,
    pub confidence_level: String,
    pub confidence_note: String,
    pub rubric_score: f64,
    pub holistic_score: f64,
    pub task_completion_score: Option<f64>,
    pub core_scores_json: String,
    pub subdimension_scores_json: String,
    pub strengths_json: String,
    pub issues_json: String,
    pub suggestions_json: String,
    pub evidence_json: String,
    pub evaluator_summary: String,
    pub markdown_cache: Option<String>,
    pub raw_json: String,
    pub created_at: String,
}

impl FinalReport {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO final_reports (
                session_id, total_score, confidence_level, confidence_note, rubric_score,
                holistic_score, task_completion_score, core_scores_json, subdimension_scores_json,
                strengths_json, issues_json, suggestions_json, evidence_json, evaluator_summary,
                markdown_cache, raw_json, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                &self.session_id, self.total_score, &self.confidence_level, &self.confidence_note,
                self.rubric_score, self.holistic_score, &self.task_completion_score,
                &self.core_scores_json, &self.subdimension_scores_json, &self.strengths_json,
                &self.issues_json, &self.suggestions_json, &self.evidence_json,
                &self.evaluator_summary, &self.markdown_cache, &self.raw_json, &self.created_at
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入final_reports记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_by_session_id(conn: &Connection, session_id: &str) -> Result<Option<FinalReport>, CommandError> {
        conn.query_row(
            "SELECT * FROM final_reports WHERE session_id = ?1",
            params![session_id],
            |row| {
                Ok(FinalReport {
                    session_id: row.get(0)?,
                    total_score: row.get(1)?,
                    confidence_level: row.get(2)?,
                    confidence_note: row.get(3)?,
                    rubric_score: row.get(4)?,
                    holistic_score: row.get(5)?,
                    task_completion_score: row.get(6)?,
                    core_scores_json: row.get(7)?,
                    subdimension_scores_json: row.get(8)?,
                    strengths_json: row.get(9)?,
                    issues_json: row.get(10)?,
                    suggestions_json: row.get(11)?,
                    evidence_json: row.get(12)?,
                    evaluator_summary: row.get(13)?,
                    markdown_cache: row.get(14)?,
                    raw_json: row.get(15)?,
                    created_at: row.get(16)?,
                })
            }
        ).optional().map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法读取final_reports记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })
    }
}

// ==================== MarkdownExport DAO ====================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownExport {
    pub id: String,
    pub session_id: String,
    pub export_path: String,
    pub export_format: String, // 'markdown'
    pub created_at: String,
}

impl MarkdownExport {
    pub fn insert(&self, conn: &Connection) -> Result<(), CommandError> {
        conn.execute(
            "INSERT INTO markdown_exports (id, session_id, export_path, export_format, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&self.id, &self.session_id, &self.export_path, &self.export_format, &self.created_at],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法插入markdown_exports记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_by_session_id(conn: &Connection, session_id: &str) -> Result<Vec<MarkdownExport>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM markdown_exports WHERE session_id = ?1 ORDER BY created_at DESC"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let rows = stmt.query_map(params![session_id], |row| {
            Ok(MarkdownExport {
                id: row.get(0)?,
                session_id: row.get(1)?,
                export_path: row.get(2)?,
                export_format: row.get(3)?,
                created_at: row.get(4)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询markdown_exports: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut exports = Vec::new();
        for row in rows {
            exports.push(row.map_err(|e| CommandError {
                code: "DB_READ_FAILED".into(),
                message: format!("无法解析markdown_exports记录: {}", e),
                severity: crate::Severity::Error,
                recoverable: false,
                action: crate::Action::CloseDialog,
            })?);
        }
        Ok(exports)
    }
}

// ==================== AssessmentTask Extension Methods ====================

impl AssessmentTask {
    pub fn get_next_unsubmitted(conn: &Connection, session_id: &str) -> Result<Option<AssessmentTask>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM assessment_tasks WHERE session_id = ?1 AND user_prompt IS NULL ORDER BY sequence_no LIMIT 1"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut rows = stmt.query_map(params![session_id], |row| {
            Ok(AssessmentTask {
                id: row.get(0)?,
                session_id: row.get(1)?,
                sequence_no: row.get(2)?,
                source: row.get(3)?,
                task_type: row.get(4)?,
                title: row.get(5)?,
                visible_task: row.get(6)?,
                visible_requirements_json: row.get(7)?,
                raw_question_json: row.get(8)?,
                hidden_reference_json: row.get(9)?,
                dimension_focus_json: row.get(10)?,
                user_prompt: row.get(11)?,
                executor_output: row.get(12)?,
                prompt_score: row.get(13)?,
                execution_score: row.get(14)?,
                final_score: row.get(15)?,
                created_at: row.get(16)?,
                completed_at: row.get(17)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询assessment_tasks: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        rows.next().transpose().map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法解析assessment_tasks记录: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })
    }

    pub fn get_by_id(conn: &Connection, task_id: &str) -> Result<Option<AssessmentTask>, CommandError> {
        conn.query_row(
            "SELECT * FROM assessment_tasks WHERE id = ?1",
            params![task_id],
            |row| {
                Ok(AssessmentTask {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    sequence_no: row.get(2)?,
                    source: row.get(3)?,
                    task_type: row.get(4)?,
                    title: row.get(5)?,
                    visible_task: row.get(6)?,
                    visible_requirements_json: row.get(7)?,
                    raw_question_json: row.get(8)?,
                    hidden_reference_json: row.get(9)?,
                    dimension_focus_json: row.get(10)?,
                    user_prompt: row.get(11)?,
                    executor_output: row.get(12)?,
                    prompt_score: row.get(13)?,
                    execution_score: row.get(14)?,
                    final_score: row.get(15)?,
                    created_at: row.get(16)?,
                    completed_at: row.get(17)?,
                })
            }
        ).optional().map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询assessment_tasks: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })
    }

    pub fn count_completed_by_session(conn: &Connection, session_id: &str) -> Result<i32, CommandError> {
        conn.query_row(
            "SELECT COUNT(*) FROM assessment_tasks WHERE session_id = ?1 AND user_prompt IS NOT NULL",
            params![session_id],
            |row| row.get(0),
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法统计已完成题目: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })
    }

    pub fn update_score(
        conn: &Connection,
        task_id: &str,
        user_prompt: &str,
        prompt_score: Option<f64>,
        execution_score: Option<f64>,
        final_score: Option<f64>,
        executor_output: Option<&str>,
    ) -> Result<(), CommandError> {
        conn.execute(
            "UPDATE assessment_tasks SET user_prompt = ?1, prompt_score = ?2, execution_score = ?3, final_score = ?4, executor_output = ?5, completed_at = ?6 WHERE id = ?7",
            params![
                user_prompt,
                prompt_score,
                execution_score,
                final_score,
                executor_output,
                &chrono::Utc::now().to_rfc3339(),
                task_id,
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法更新题目评分: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }
}

// ==================== AssessmentSession Extension Methods ====================

impl AssessmentSession {
    pub fn update_status(conn: &Connection, session_id: &str, status: &str, total_score: Option<f64>, confidence_level: Option<&str>, confidence_note: Option<&str>) -> Result<(), CommandError> {
        let now = chrono::Utc::now().to_rfc3339();
        let completed_at: Option<String> = if status == "completed" { Some(now.clone()) } else { None };
        
        conn.execute(
            "UPDATE assessment_sessions SET status = ?1, total_score = ?2, confidence_level = ?3, confidence_note = ?4, completed_at = ?5, updated_at = ?6 WHERE id = ?7",
            params![
                status,
                total_score,
                confidence_level,
                confidence_note,
                completed_at,
                &now,
                session_id,
            ],
        ).map_err(|e| CommandError {
            code: "DB_WRITE_FAILED".into(),
            message: format!("无法更新会话状态: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        Ok(())
    }

    pub fn get_all_by_status(conn: &Connection, status: &str) -> Result<Vec<AssessmentSession>, CommandError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM assessment_sessions WHERE status = ?1 ORDER BY created_at DESC"
        ).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法准备查询: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let rows = stmt.query_map(params![status], |row| {
            Ok(AssessmentSession {
                id: row.get(0)?,
                scope: row.get(1)?,
                interaction_mode: row.get(2)?,
                question_source: row.get(3)?,
                static_strategy: row.get(4)?,
                dynamic_end_strategy: row.get(5)?,
                fixed_round_limit: row.get(6)?,
                task_type: row.get(7)?,
                generation_constraints: row.get(8)?,
                advanced_eval_context: row.get(9)?,
                executor_model_snapshot_json: row.get(10)?,
                evaluator_model_snapshot_json: row.get(11)?,
                generator_model_snapshot_json: row.get(12)?,
                total_score: row.get(13)?,
                confidence_level: row.get(14)?,
                confidence_note: row.get(15)?,
                question_count: row.get(16)?,
                effective_round_count: row.get(17)?,
                status: row.get(18)?,
                rubric_version: row.get(19)?,
                prompt_bundle_version: row.get(20)?,
                question_bundle_version: row.get(21)?,
                created_at: row.get(22)?,
                completed_at: row.get(23)?,
                updated_at: row.get(24)?,
            })
        }).map_err(|e| CommandError {
            code: "DB_READ_FAILED".into(),
            message: format!("无法查询assessment_sessions: {}", e),
            severity: crate::Severity::Error,
            recoverable: false,
            action: crate::Action::CloseDialog,
        })?;
        
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row.map_err(|e| CommandError {
                code: "DB_READ_FAILED".into(),
                message: format!("无法解析assessment_sessions记录: {}", e),
                severity: crate::Severity::Error,
                recoverable: false,
                action: crate::Action::CloseDialog,
            })?);
        }
        Ok(sessions)
    }
}