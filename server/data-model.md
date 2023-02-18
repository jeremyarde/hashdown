# Data model

Form
- questions[]
- answers[]
- views
- starts
- submissions
- completions
- time
- created by
- modified on
- id

Question
- id
- created_on
- modified_on
- type
- value

Answer
- id
- question id
- form id
- submit date
- question id: answer[]
- filled by details
  - referrer website?
  - country?



Flow
1. write text -> parse into form -> questions, question types
2. Save to database
3. 