use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct QuestionSchema {
    pub id: String,
    pub source: String, // "builtin" or "ai_generated"
    pub assessment_mode: String, // "static" or "dynamic"
    pub task_type: String, // "daily_qa", "writing", etc.
    pub scope_fit: Vec<String>, // ["fun"], ["full"], or both
    pub title: String,
    pub version_info: VersionInfo,
    pub visible_prompt: VisiblePrompt,
    pub hidden_reference: HiddenReference,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct VersionInfo {
    pub question_schema_version: String,
    pub question_version: String,
    pub rubric_version: String,
    pub generator_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct VisiblePrompt {
    pub task: String,
    pub requirements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct HiddenReference {
    pub dimension_focus: DimensionFocus,
    pub ideal_elements: Vec<String>,
    pub completion_criteria: Vec<String>,
    pub notes_for_evaluator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_done_signals: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct DimensionFocus {
    pub clarity_output: u32,
    pub structuring: u32,
    pub context_constraints: u32,
    pub gap_risk: u32,
    pub iteration_acceptance: u32,
}

impl QuestionSchema {
    /// Validate the question schema
    pub fn validate(&self) -> Result<(), String> {
        // Check dimension focus sum = 100
        let sum = self.hidden_reference.dimension_focus.clarity_output
            + self.hidden_reference.dimension_focus.structuring
            + self.hidden_reference.dimension_focus.context_constraints
            + self.hidden_reference.dimension_focus.gap_risk
            + self.hidden_reference.dimension_focus.iteration_acceptance;
        
        if sum != 100 {
            return Err(format!("Dimension focus sum must be 100, got {}", sum));
        }
        
        // Check requirements not empty
        if self.visible_prompt.requirements.is_empty() {
            return Err("Visible prompt requirements cannot be empty".to_string());
        }
        
        // Check task_done_signals only for dynamic mode
        if self.assessment_mode == "dynamic" && self.hidden_reference.task_done_signals.is_none() {
            return Err("Dynamic questions must have task_done_signals".to_string());
        }
        if self.assessment_mode == "static" && self.hidden_reference.task_done_signals.is_some() {
            return Err("Static questions should not have task_done_signals".to_string());
        }
        
        Ok(())
    }
}

/// Load all built-in questions from the question-bank directory
pub fn load_builtin_questions() -> Result<Vec<QuestionSchema>, String> {
    let mut questions = Vec::new();
    
    // Determine the path to question-bank directory
    // In development, it's in the project root (../question-bank from src-tauri/src)
    // In production, it might be embedded or in resources
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bank_root = project_root.join("../question-bank");
    
    if !bank_root.exists() {
        return Err(format!("Question bank directory not found at {:?}", bank_root));
    }
    
    // Load static questions
    let static_dir = bank_root.join("static");
    if static_dir.exists() {
        load_questions_from_dir(&static_dir, &mut questions)?;
    }
    
    // Load dynamic questions
    let dynamic_dir = bank_root.join("dynamic");
    if dynamic_dir.exists() {
        load_questions_from_dir(&dynamic_dir, &mut questions)?;
    }
    
    Ok(questions)
}

fn load_questions_from_dir(dir: &Path, questions: &mut Vec<QuestionSchema>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
            let question: QuestionSchema = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON from {:?}: {}", path, e))?;
            
            // Validate
            question.validate()
                .map_err(|e| format!("Invalid question {}: {}", question.id, e))?;
            
            questions.push(question);
        }
    }
    
    Ok(())
}

/// Get questions filtered by assessment mode and scope
pub fn filter_questions(
    questions: &[QuestionSchema],
    assessment_mode: &str,
    scope: &str,
) -> Vec<QuestionSchema> {
    questions
        .iter()
        .filter(|q| q.assessment_mode == assessment_mode && q.scope_fit.contains(&scope.to_string()))
        .cloned()
        .collect()
}

/// Group questions by task type
pub fn group_by_task_type(questions: &[QuestionSchema]) -> HashMap<String, Vec<QuestionSchema>> {
    let mut map = HashMap::new();
    for q in questions {
        map.entry(q.task_type.clone()).or_insert_with(Vec::new).push(q.clone());
    }
    map
}