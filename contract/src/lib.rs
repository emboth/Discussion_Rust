use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    content: String,
    responder_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Question {
    content: String,
    asker_id: AccountId,
    id: u128,
    answers: Vec<Answer>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Discussion {
    questions_asked: UnorderedMap<u128, Question>,
    number_of_questions: u128,
}

impl Default for Discussion {
    fn default() -> Self {
        Self {
            questions_asked: UnorderedMap::new(b'm'),
            number_of_questions: 0,
        }
    }
}

#[near_bindgen]
impl Discussion {
    pub fn add_question(&mut self, content: String) -> Question {
        let question = Question {
            content: content,
            asker_id: env::signer_account_id(),
            id: self.number_of_questions,
            answers: Vec::<Answer>::new(),
        };
        self.questions_asked
            .insert(&self.number_of_questions, &question);
        self.number_of_questions += 1;
        return question;
    }

    pub fn answer_question(&mut self, question_id: u128, answer_content: String) -> Question {
        let mut question;
        if let None = self.questions_asked.get(&question_id) {
            question = Question {
                content: "No question at that ID".to_string(),
                asker_id: env::signer_account_id(),
                id: question_id,
                answers: Vec::<Answer>::new(),
            };
            return question;
        } else {
            question = self.questions_asked.get(&question_id).unwrap();
        }

        let answer = Answer {
            content: answer_content,
            responder_id: env::signer_account_id(),
        };

        question.answers.push(answer);
        self.questions_asked.insert(&question_id, &question);
        return question;
    }

    pub fn get_question_by_id(&self, question_id: u128) -> Question {
        if let None = self.questions_asked.get(&question_id) {
            return Question {
                content: "No question at that ID".to_string(),
                asker_id: "emilbob.testnet".parse().unwrap(),
                id: question_id,
                answers: Vec::<Answer>::new(),
            };
        } else {
            return self.questions_asked.get(&question_id).unwrap();
        }
    }

    pub fn get_all_questions(&self) -> Vec<(u128, Question)> {
        if self.questions_asked.is_empty() {
            return Vec::<(u128, Question)>::new();
        }

        return self.questions_asked.to_vec();
    }
}
