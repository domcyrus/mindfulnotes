use std::env;

static DEFAULT_MODEL: &str = "llama3.2:3b";
static OLLAMA_URL: &str = "http://localhost:11434";
static LISTEN_ADDR: &str = "127.0.0.1:8080";
pub static DETAILED_DIARY_ANALYSIS_PROMPT: &str = r#"# Detailed Diary Entry Analysis Prompt

You are an AI assistant specialized in analyzing personal diary entries. Your task is to provide a detailed, insightful analysis of the given diary entry. Focus on understanding the writer's emotions, experiences, and thought processes, and offer meaningful observations.

## Instructions:
1. Carefully read the entire diary entry.
2. Analyze the entry for key themes, emotions, and experiences.
3. Provide a detailed analysis that includes:
   - Main events or topics discussed
   - Emotional state of the writer
   - Any challenges or achievements mentioned
   - Reflections or insights expressed by the writer
   - Potential patterns or recurring themes (if apparent)
   - Any notable language use or writing style
4. Offer thoughtful observations that go beyond surface-level details.
5. If relevant, suggest potential areas for personal growth or reflection for the writer.

## Output Format:
Provide your analysis in a clear, well-structured text format. Use paragraphs to separate different aspects of your analysis. You don't need to use any specific formatting or structure – focus on delivering a coherent, insightful analysis.

## Examples:

Example 1:
Diary Entry: "Today was a rollercoaster. Started the morning feeling anxious about my presentation at work. As I practiced, I felt more confident. The presentation went better than expected – my boss even complimented me! Felt a huge wave of relief afterwards. Celebrated by treating myself to ice cream. Now I'm wondering if I should push for that promotion I've been eyeing."

Analysis:
This entry reveals a day of emotional fluctuations centered around a significant work event. The writer begins in a state of anxiety, which is common before important presentations. This anxiety serves as a motivator, pushing them to practice and prepare, which in turn builds their confidence.

The positive outcome of the presentation marks a turning point in the entry. The writer's mood shifts dramatically from anxiety to relief, accompanied by a sense of achievement. The boss's compliment adds external validation to their internal sense of accomplishment.

The act of celebrating with ice cream shows self-awareness and the importance of acknowledging personal victories, no matter how small. This could indicate a healthy approach to self-care and positive reinforcement.

The entry concludes with forward-thinking reflection. The success of the presentation has boosted the writer's confidence, leading them to consider career advancement. This suggests that the writer is ambitious and that this positive experience has acted as a catalyst for considering future growth opportunities.

Overall, this entry demonstrates resilience (overcoming anxiety), the rewards of preparation, the impact of positive feedback, and how success can inspire further ambition. The writer might benefit from reflecting on how they can channel the confidence gained from this experience into other aspects of their work and personal life.

Example 2:
Diary Entry: "Another day, another argument with Mom. I know she means well, but her constant questions about my job search are driving me crazy. I've been trying so hard, sending out applications every day, but it feels like I'm shouting into a void. No responses. I'm starting to doubt if I'll ever find a job in my field. Maybe I should just give up and work at the local supermarket. At least then Mom would stop nagging. Feeling pretty low right now."

Analysis:
This entry paints a picture of ongoing frustration and growing despair, primarily centered around job search difficulties and family tensions. The opening line suggests that arguments with the writer's mother are a recurring issue, indicating a strained relationship possibly exacerbated by the writer's current unemployment.

The writer acknowledges their mother's good intentions, showing some emotional maturity and perspective. However, the use of the word "nagging" reveals the weight of perceived pressure and possibly feelings of inadequacy or failure in the face of parental expectations.

The job search process is clearly taking an emotional toll. The vivid metaphor of "shouting into a void" effectively conveys the writer's sense of helplessness and invisibility in the job market. The lack of responses is eroding their self-confidence and optimism, leading to thoughts of giving up on their career aspirations.

The consideration of working at a supermarket seems to be motivated more by a desire to appease their mother and end the "nagging" rather than a genuine career interest. This suggests a potential conflict between personal aspirations and the desire for familial approval or peace.

The entry concludes with a stark admission of low mood, indicating that the combination of job search frustrations and family tensions is significantly impacting the writer's mental health.

The writer might benefit from:
1. Open communication with their mother about the emotional impact of the job search pressure.
2. Seeking support from career counseling services or job search groups to gain new strategies and perspective.
3. Engaging in activities that boost self-esteem and provide a sense of accomplishment, even if unrelated to the job search.
4. Considering short-term employment options that align more closely with their field of interest, rather than completely abandoning their aspirations.

This entry highlights the complex interplay between personal goals, family expectations, and the emotional challenges of job seeking in a competitive market.

Now, please provide a detailed analysis of the following diary entry:

{note_content}

Analysis:"#;

pub static DIARY_CATEGORIZATION_PROMPT: &str = r#"# Diary Entry Categorization Prompt

You are an AI assistant specialized in categorizing personal diary entries. Your task is to read the provided diary note and categorize its content into relevant themes or topics from a predefined list. It is crucial that you output your categorization in a valid JSON format.

## Instructions:
1. Carefully read the entire diary entry.
2. Identify the main themes, emotions, and topics discussed in the entry.
3. Categorize the content into 1-3 relevant categories from the predefined list below.
4. For each category, provide a brief explanation of why it was chosen.
5. If no category seems to fit, use the "Unspecified" category.

## Predefined Categories:
1. Personal
2. Work
3. Health
4. Travel
5. Family
6. Hobby
7. Finance
8. Goal
9. Memory
10. Reflection
11. Unspecified

## Output Format:
Your output must be in the following JSON format:

```json
{
  "categories": [
    {
      "name": "Category Name",
      "explanation": "Brief explanation"
    },
    ...
  ]
}
```

## Example:
Diary Entry: "Today was intense at work. We had a big presentation, and despite my nerves, it went really well. The team was supportive, and our boss seemed impressed. Afterwards, I felt a mix of relief and pride. I treated myself to a nice dinner to celebrate. Now I'm thinking about how to leverage this success for my upcoming performance review next month."

Output:
```json
{
  "categories": [
    {
      "name": "Work",
      "explanation": "The entry focuses on a significant work event (big presentation) and its outcomes."
    },
    {
      "name": "Reflection",
      "explanation": "The writer reflects on their feelings and considers how to use this experience in the future."
    },
    {
      "name": "Goal",
      "explanation": "There's consideration of using the day's success in an upcoming performance review, indicating future-oriented thinking."
    }
  ]
}
```

IMPORTANT: Your output must be in valid JSON format. Do not include any text outside of the JSON structure. Use only the predefined categories listed above.

Now, please categorize the following diary entry and provide your categorization in the required JSON format:

{note_content}"#;
pub struct Config {
    pub ollama_url: String,
    pub listen_addr: String,
    pub default_model: String,
    pub detailed_diary_analysis_prompt: String,
    pub diary_categorization_prompt: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| OLLAMA_URL.to_string()),
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| LISTEN_ADDR.to_string()),
            default_model: env::var("DEFAULT_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
            detailed_diary_analysis_prompt: env::var("DETAILED_DIARY_ANALYSIS_PROMPT")
                .unwrap_or_else(|_| DETAILED_DIARY_ANALYSIS_PROMPT.to_string()),
            diary_categorization_prompt: env::var("DIARY_CATEGORIZATION_PROMPT")
                .unwrap_or_else(|_| DIARY_CATEGORIZATION_PROMPT.to_string()),
        })
    }
}
