// Assessment question content for all assessment types
use super::models::AssessmentQuestion;

/// PHQ-9 Questions (Patient Health Questionnaire-9)
/// Depression screening tool with 9 questions, 0-3 scale
pub fn get_phq9_questions() -> Vec<AssessmentQuestion> {
    vec![
        AssessmentQuestion {
            number: 1,
            text: "Little interest or pleasure in doing things".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 2,
            text: "Feeling down, depressed, or hopeless".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 3,
            text: "Trouble falling or staying asleep, or sleeping too much".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 4,
            text: "Feeling tired or having little energy".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 5,
            text: "Poor appetite or overeating".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 6,
            text: "Feeling bad about yourself - or that you are a failure or have let yourself or your family down".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 7,
            text: "Trouble concentrating on things, such as reading the newspaper or watching television".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 8,
            text: "Moving or speaking so slowly that other people could have noticed. Or the opposite - being so fidgety or restless that you have been moving around a lot more than usual".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 9,
            text: "Thoughts that you would be better off dead, or of hurting yourself in some way".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
    ]
}

/// GAD-7 Questions (Generalized Anxiety Disorder-7)
/// Anxiety screening tool with 7 questions, 0-3 scale
pub fn get_gad7_questions() -> Vec<AssessmentQuestion> {
    vec![
        AssessmentQuestion {
            number: 1,
            text: "Feeling nervous, anxious, or on edge".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 2,
            text: "Not being able to stop or control worrying".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 3,
            text: "Worrying too much about different things".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 4,
            text: "Trouble relaxing".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 5,
            text: "Being so restless that it is hard to sit still".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 6,
            text: "Becoming easily annoyed or irritable".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 7,
            text: "Feeling afraid, as if something awful might happen".to_string(),
            options: vec![
                "Not at all".to_string(),
                "Several days".to_string(),
                "More than half the days".to_string(),
                "Nearly every day".to_string(),
            ],
        },
    ]
}

/// CES-D Questions (Center for Epidemiologic Studies Depression Scale)
/// Depression assessment with 20 questions, 0-3 scale
pub fn get_cesd_questions() -> Vec<AssessmentQuestion> {
    let options = vec![
        "Rarely or none of the time (less than 1 day)".to_string(),
        "Some or a little of the time (1-2 days)".to_string(),
        "Occasionally or a moderate amount of time (3-4 days)".to_string(),
        "Most or all of the time (5-7 days)".to_string(),
    ];

    vec![
        AssessmentQuestion {
            number: 1,
            text: "I was bothered by things that usually don't bother me".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 2,
            text: "I did not feel like eating; my appetite was poor".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 3,
            text: "I felt that I could not shake off the blues even with help from my family or friends".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 4,
            text: "I felt I was just as good as other people".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 5,
            text: "I had trouble keeping my mind on what I was doing".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 6,
            text: "I felt depressed".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 7,
            text: "I felt that everything I did was an effort".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 8,
            text: "I felt hopeful about the future".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 9,
            text: "I thought my life had been a failure".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 10,
            text: "I felt fearful".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 11,
            text: "My sleep was restless".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 12,
            text: "I was happy".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 13,
            text: "I talked less than usual".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 14,
            text: "I felt lonely".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 15,
            text: "People were unfriendly".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 16,
            text: "I enjoyed life".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 17,
            text: "I had crying spells".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 18,
            text: "I felt sad".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 19,
            text: "I felt that people dislike me".to_string(),
            options: options.clone(),
        },
        AssessmentQuestion {
            number: 20,
            text: "I could not get going".to_string(),
            options: options.clone(),
        },
    ]
}

/// OASIS Questions (Overall Anxiety Severity and Impairment Scale)
/// Anxiety assessment with 5 questions, 0-4 scale
pub fn get_oasis_questions() -> Vec<AssessmentQuestion> {
    vec![
        AssessmentQuestion {
            number: 1,
            text: "In the past week, how often have you felt anxious?".to_string(),
            options: vec![
                "No anxiety in the past week".to_string(),
                "Infrequent anxiety. Felt anxious a few times".to_string(),
                "Frequent anxiety. Felt anxious most of the time".to_string(),
                "Constant anxiety. Felt anxious all of the time".to_string(),
                "Extreme anxiety. Felt anxious every moment".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 2,
            text: "In the past week, when you have felt anxious, how intense or severe was your anxiety?".to_string(),
            options: vec![
                "No anxiety".to_string(),
                "Mild anxiety. Minimally distressing".to_string(),
                "Moderate anxiety. Distressing, but manageable".to_string(),
                "Severe anxiety. Difficult to tolerate".to_string(),
                "Extreme anxiety. Barely tolerable, overwhelming".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 3,
            text: "In the past week, how often did you avoid situations, places, objects, or activities because of anxiety or fear?".to_string(),
            options: vec![
                "Never avoided".to_string(),
                "Infrequently avoided. Avoided a few times".to_string(),
                "Occasionally avoided. Avoided about half the time".to_string(),
                "Frequently avoided. Avoided most of the time".to_string(),
                "All the time. Constantly avoided situations".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 4,
            text: "In the past week, how much did your anxiety interfere with your ability to do the things you needed to do at work, at school, or at home?".to_string(),
            options: vec![
                "No interference".to_string(),
                "Mild interference. Slightly interfered".to_string(),
                "Moderate interference. Definitely interfered but still manageable".to_string(),
                "Severe interference. Substantially interfered".to_string(),
                "Extreme interference. Completely interfered. Unable to do tasks".to_string(),
            ],
        },
        AssessmentQuestion {
            number: 5,
            text: "In the past week, how much has anxiety interfered with your social life and relationships?".to_string(),
            options: vec![
                "No interference".to_string(),
                "Mild interference. Slightly interfered".to_string(),
                "Moderate interference. Definitely interfered but still manageable".to_string(),
                "Severe interference. Substantially interfered".to_string(),
                "Extreme interference. Completely interfered. Unable to maintain relationships".to_string(),
            ],
        },
    ]
}
