use leptos::{web_sys::Event, *};
use leptos_meta::*;
use markdownparser::{Question, Questions};

#[derive(Copy, Clone)]
struct QuestionsContext(ReadSignal<Questions>);

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());
    // let (content, set_content) = create_signal(cx, "");
    let (questions, set_questions) = create_signal(cx, vec![]);
    let content_input = NodeRef::new(cx);
    // let sample_q = Question {
    //     id: 0,
    //     text: String::from("tests"),
    //     options: vec![],
    // };
    provide_context(cx, QuestionsContext(questions));
    // let mut questions: Vec<Question> = vec![];

    let mut handle_input_change = move |ev| {
        let string_content = content_input
            .get()
            .expect("input should contain something")
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value();
        log!("content: {:?}", string_content);
        let results = markdownparser::parse_markdown_blocks(string_content);
        // set_questions.update(|x| {
        //     x.clear();
        //     // x.extend(results);
        // });

        log!("parse results: {:?}", questions.get());
    };

    view! {
        cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
        <Stylesheet id="leptos" href="/style.css"/>
        <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
        <div>
        // <h2>{"TODO"}</h2>
        <textarea
            type="text"         // attributes work just like they do in HTML
            name="new-todo"
            prop:value=""
            value="initial"     // side note: the DOM `value` attribute only sets *initial* value
                                // this is very important when working with forms!
            _ref=content_input // `_ref` stores tis element in a variable
            on:input=move |ev| handle_input_change(ev)
        />
        <QuestionsComponent/>
        </div>
        </main>
    }
}

#[component]
fn QuestionsComponent(cx: Scope) -> Element {
    let questions = use_context::<QuestionsContext>(cx).unwrap().0;
    log!("questions - {:?}", questions.get());
    let temp = Question {
        id: 0,
        text: String::from("tests"),
        options: vec![],
    };

    // let first_q = questions.get().get(0).unwrap_or(&temp);

    let question_list = move || {
        questions
            .get()
            .into_iter()
            .map(|x| view! {cx, <li>{x.text}</li>})
            .collect::<Vec<_>>()
    };
    // log!("firs question: {:?}", &first_q);

    view! {
        cx,
        <div>
        <h2>"This is the questions section"</h2>
        // <p>{questions.iter().map(|x: Question| {view! {x.text}})}</p>
        <ul>
            {question_list}
        </ul>

        </div>
    }
}

// #[component]
// fn SingleQuestion(cx: Scope) -> Element {
//     let question = Question {
//         id: 0,
//         text: String::from("sample q"),
//         options: vec![],
//     };
//     view! {
//         cx, <li>
//         <div>
//         <h3>{question.text}</h3>
//         </div></li>
//     }
// }
