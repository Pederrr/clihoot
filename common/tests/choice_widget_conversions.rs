use uuid::Uuid;

use common::questions::{Choice, ChoiceCensored, Question, QuestionCensored};
use common::terminal::widgets::choice::{ChoiceGrid, ChoiceItem};

#[test]
fn test_choice_item_from_censored() {
    let id = Uuid::new_v4();
    let text = "Test string".to_string();
    let choice = ChoiceCensored {
        id,
        text: text.clone(),
    };

    let choice_item: ChoiceItem = choice.into();

    let wanted = ChoiceItem::new(text.clone(), false, id);

    assert!(choice_item == wanted);
}

#[test]
fn test_choice_item_from_uncensored() {
    let id = Uuid::new_v4();
    let text = "Test string".to_string();
    let choice = Choice {
        id,
        is_right: true,
        text: text.clone(),
    };

    let choice_item: ChoiceItem = choice.into();

    let wanted = ChoiceItem::new(text.clone(), true, id);

    assert!(choice_item == wanted);
}

#[test]
fn test_choice_grid_from_censored_2_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = QuestionCensored {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            ChoiceCensored {
                id: id_1,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_2,
                text: text.clone(),
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(text.clone(), false, id_1),
        ChoiceItem::new(text.clone(), false, id_2),
    ]]);

    assert!(choice_grid == wanted);
}

#[test]
fn test_choice_grid_from_censored_3_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let id_3 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = QuestionCensored {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            ChoiceCensored {
                id: id_1,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_2,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_3,
                text: text.clone(),
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(text.clone(), false, id_1),
        ChoiceItem::new(text.clone(), false, id_2),
        ChoiceItem::new(text.clone(), false, id_3),
    ]]);

    assert!(choice_grid == wanted);
}

#[test]
fn test_choice_grid_from_censored_4_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let id_3 = Uuid::new_v4();
    let id_4 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = QuestionCensored {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            ChoiceCensored {
                id: id_1,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_2,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_3,
                text: text.clone(),
            },
            ChoiceCensored {
                id: id_4,
                text: text.clone(),
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![
        vec![
            ChoiceItem::new(text.clone(), false, id_1),
            ChoiceItem::new(text.clone(), false, id_2),
        ],
        vec![
            ChoiceItem::new(text.clone(), false, id_3),
            ChoiceItem::new(text.clone(), false, id_4),
        ],
    ]);

    assert!(choice_grid == wanted);
}

#[test]
fn test_choice_grid_from_uncensored_2_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = Question {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            Choice {
                id: id_1,
                text: text.clone(),
                is_right: true,
            },
            Choice {
                id: id_2,
                text: text.clone(),
                is_right: false,
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(text.clone(), true, id_1),
        ChoiceItem::new(text.clone(), false, id_2),
    ]]);

    assert!(choice_grid == wanted);
}

#[test]
fn test_choice_grid_from_uncensored_3_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let id_3 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = Question {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            Choice {
                id: id_1,
                text: text.clone(),
                is_right: false,
            },
            Choice {
                id: id_2,
                text: text.clone(),
                is_right: false,
            },
            Choice {
                id: id_3,
                text: text.clone(),
                is_right: true,
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![vec![
        ChoiceItem::new(text.clone(), false, id_1),
        ChoiceItem::new(text.clone(), false, id_2),
        ChoiceItem::new(text.clone(), true, id_3),
    ]]);

    assert!(choice_grid == wanted);
}

#[test]
fn test_choice_grid_from_uncensored_4_choices() {
    let id_1 = Uuid::new_v4();
    let id_2 = Uuid::new_v4();
    let id_3 = Uuid::new_v4();
    let id_4 = Uuid::new_v4();
    let text = "Test string".to_string();

    let question = Question {
        text: "Why are we here?".to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            Choice {
                id: id_1,
                text: text.clone(),
                is_right: false,
            },
            Choice {
                id: id_2,
                text: text.clone(),
                is_right: true,
            },
            Choice {
                id: id_3,
                text: text.clone(),
                is_right: true,
            },
            Choice {
                id: id_4,
                text: text.clone(),
                is_right: false,
            },
        ],
    };

    let choice_grid: ChoiceGrid = question.into();

    let wanted = ChoiceGrid::new(vec![
        vec![
            ChoiceItem::new(text.clone(), false, id_1),
            ChoiceItem::new(text.clone(), true, id_2),
        ],
        vec![
            ChoiceItem::new(text.clone(), true, id_3),
            ChoiceItem::new(text.clone(), false, id_4),
        ],
    ]);

    assert!(choice_grid == wanted);
}
