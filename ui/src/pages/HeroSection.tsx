import { useState } from "react";
import { RenderedForm } from "../components/custom/RenderedForm.tsx";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";

const heading = (
  <div>
    <h1
      className="flex justify-center pt-4 text-4xl text-center top-10"
      style={{ fontWeight: "700", color: "black" }}
    >
      Create forms and surveys with ease - using just text.
    </h1>
  </div>
);

const subheading = (
  <div>
    <span className="text-xl">
      <h2>Turn simple text into powerful forms.</h2>
      At Hashdown, we believe creating forms and surveys should be effortless.
      With our unique system, all you need is a subset of Markdown to build
      dynamic forms—no coding required.
    </span>
  </div>
);

const features = (
  <ul>
    <li>
      Markdown-Driven: Use familiar Markdown syntax to create forms and surveys
      in seconds.
    </li>
    <li>
      Instant Sharing: Get a shareable URL instantly—no extra setup needed.
    </li>
    <li>
      API Ready: Prefer automation? Use our API to integrate form creation
      directly into your workflow.
    </li>
  </ul>
);

const howitworks = <></>;

const faqs = (
  <div className="">
    <h2>Frequently Asked Questions</h2>
    <div className="faq-section">
      <div className="faq-item">
        <input type="checkbox" id="faq1" />
        <label htmlFor="faq1" className="">
          <h3>1. What is Hashdown?</h3>
        </label>
        <div className="faq-content">
          <p>
            Hashdown is a tool that allows you to create forms and surveys using
            a simple subset of Markdown syntax. You can generate forms quickly
            through text, share them via a URL, or integrate them via our API.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq2" />
        <label htmlFor="faq2" className="">
          <h3>2. Do I need to know how to code to use Hashdown?</h3>
        </label>
        <div className="faq-content">
          <p>
            No coding is required! Hashdown leverages easy-to-learn Markdown
            syntax, making form creation accessible to anyone familiar with
            basic text formatting.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq3" />
        <label htmlFor="faq3" className="">
          <h3>3. What types of forms can I create?</h3>
        </label>
        <div className="faq-content">
          <p>
            You can create a variety of forms, including surveys, feedback
            forms, contact forms, registration forms, and more. If your use case
            involves collecting information, Hashdown has you covered.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq4" />
        <label htmlFor="faq4" className="">
          <h3>4. How do I share the forms I create?</h3>
        </label>
        <div className="faq-content">
          <p>
            Once you create a form, Hashdown generates a unique URL that you can
            share anywhere—via email, social media, or embedded on your website.
            Users can access and fill out the form through that link.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq5" />
        <label htmlFor="faq5" className="">
          <h3>5. Can I embed the forms on my website?</h3>
        </label>
        <div className="faq-content">
          <p>
            Yes, you can embed the forms directly into your website using the
            provided URL or iFrame code, ensuring they fit seamlessly with your
            existing content.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq6" />
        <label htmlFor="faq6" className="">
          <h3>6. Does Hashdown offer an API?</h3>
        </label>
        <div className="faq-content">
          <p>
            Yes, Hashdown comes with a fully functional API. You can integrate
            form creation and submission collection into your applications,
            allowing for automation and custom workflows.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq7" />
        <label htmlFor="faq7" className="">
          <h3>7. How is my data stored? Is it secure?</h3>
        </label>
        <div className="faq-content">
          <p>
            We prioritize security. All data submitted through forms is
            encrypted and stored securely on our servers. You can also export
            responses for your own record-keeping or analysis.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq8" />
        <label htmlFor="faq8" className="">
          <h3>8. Can I customize the look and feel of my forms?</h3>
        </label>
        <div className="faq-content">
          <p>
            While the default forms have a clean, simple design, you can apply
            custom CSS for more advanced styling, allowing you to match the
            forms to your brand’s visual identity.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq9" />
        <label htmlFor="faq9" className="">
          <h3>9. Is there a limit to the number of forms I can create?</h3>
        </label>
        <div className="faq-content">
          <p>
            The number of forms you can create depends on your subscription
            plan. Our free plan offers a limited number of forms, while our
            premium plans offer unlimited form creation and advanced features.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq10" />
        <label htmlFor="faq10" className="">
          <h3>10. How much does Hashdown cost?</h3>
        </label>
        <div className="faq-content">
          <p>
            We offer several pricing tiers to suit your needs. You can start for
            free, with premium features and more forms available in paid plans.
            Visit our pricing page for more details.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq11" />
        <label htmlFor="faq11" className="">
          <h3>11. Can I use Hashdown for team collaboration?</h3>
        </label>
        <div className="faq-content">
          <p>
            Yes! Our team features allow you to collaborate with others by
            sharing forms, reviewing responses, and managing form settings
            within your organization.
          </p>
        </div>
      </div>

      <div className="faq-item">
        <input type="checkbox" id="faq12" />
        <label htmlFor="faq12" className="">
          <h3>12. What kind of support do you offer?</h3>
        </label>
        <div className="faq-content">
          <p>
            We provide a detailed knowledge base and email support. Our premium
            users receive priority customer support for faster resolutions.
          </p>
        </div>
      </div>
    </div>
  </div>
);

const benefits = (
  <div className="benefits-section">
    <h2>Benefits of Using Hashdown vs. Traditional GUI-Based Form Builders</h2>

    <div className="benefit-item">
      <h3>1. Speed and Simplicity</h3>
      <p>
        With Hashdown, creating forms is as fast as typing. You don’t need to
        click through multiple menus or interfaces—just write in plain text
        using Markdown syntax. This is ideal for users who prefer a streamlined,
        no-frills approach to form creation.
      </p>
    </div>

    <div className="benefit-item">
      <h3>2. Markdown Familiarity</h3>
      <p>
        For those already familiar with Markdown, Hashdown feels intuitive. Many
        developers, writers, and content creators use Markdown daily, making it
        second nature to create forms without having to learn a new visual
        editor.
      </p>
    </div>

    <div className="benefit-item">
      <h3>3. Focus on Functionality, Not Design</h3>
      <p>
        Hashdown emphasizes function over form, so you can quickly generate
        forms without getting bogged down in design decisions. If your priority
        is collecting responses, rather than making the form look a certain way,
        Hashdown helps you cut to the chase.
      </p>
    </div>

    <div className="benefit-item">
      <h3>4. API Access and Automation</h3>
      <p>
        Traditional GUI form builders often limit the level of automation and
        integration options. Hashdown provides an API, making it ideal for
        integrating form creation into your app or automating workflows. This is
        a major advantage for developers and businesses looking to streamline
        operations.
      </p>
    </div>

    <div className="benefit-item">
      <h3>5. Lightweight and Accessible</h3>
      <p>
        Without the overhead of a full GUI, Hashdown is lightweight, making it
        faster to load and use. This is especially beneficial when working in
        environments with limited resources or when quick edits are needed.
      </p>
    </div>

    <div className="benefit-item">
      <h3>6. Flexibility for Developers</h3>
      <p>
        Developers can generate forms dynamically by writing plain text or
        through code, which allows for easier version control and scriptable
        workflows. Traditional GUI builders often have limitations when it comes
        to automation or integrating with development pipelines.
      </p>
    </div>

    <div className="benefit-item">
      <h3>7. Lower Learning Curve for Text-Based Creators</h3>
      <p>
        For users who work in text-based environments (writers, researchers,
        etc.), Hashdown fits seamlessly into their workflow. There’s no need to
        adjust to a graphical interface—they can stick to their preferred way of
        working.
      </p>
    </div>

    <div className="benefit-item">
      <h3>8. Clean and Consistent Output</h3>
      <p>
        Because forms are built from a consistent, Markdown-based syntax, the
        output is always predictable and clean. You won’t need to worry about
        design inconsistencies that can sometimes occur in drag-and-drop
        builders.
      </p>
    </div>

    <div className="benefit-item">
      <h3>9. Shareable and Embeddable</h3>
      <p>
        Hashdown instantly generates a shareable URL for your form, which can be
        shared anywhere or embedded on websites. This makes distribution simple,
        without requiring additional steps that GUI form builders might involve.
      </p>
    </div>

    <div className="benefit-item">
      <h3>10. Cost-Effective for Advanced Users</h3>
      <p>
        Since Hashdown doesn’t require the development and maintenance of a
        heavy graphical interface, it may be more cost-effective for users who
        don’t need all the design capabilities of a traditional form builder.
      </p>
    </div>
  </div>
);

export function HeroSection() {
  const [heroContent, setHeroContent] = useState(`# Feedback

text: How did you hear about us?

radio: Can we contact you for follow up questions?
- yes
- no

submit: submit`);

  let sampleSurvey = markdown_to_form_wasm_v2(heroContent);
  const surveyExample = (
    <div className="w-1/2 h-full pr-10 ">
      <ol
        style={{
          whiteSpace: "pre",
          wordWrap: "normal",
          backgroundColor: "white",
        }}
        className="flex flex-col pl-2 ml-4 bg-white border border-dashed"
      >
        {heroContent.split("\n").map((item, i) => {
          return (
            <li
              key={i}
              className="justify-between text-xl text-left min-h-6 "
              style={{
                fontSize: "1rem",
                wordWrap: "normal",
                wordBreak: "normal",
                whiteSpace: "normal",
                borderBottom: "1px dashed gray",
              }}
            >
              <div className="justify-between w-full h-full">{item}</div>
            </li>
          );
        })}
      </ol>
    </div>
  );

  return (
    <div className="flex flex-col p-6 space-y-8">
      <div className="p-6 pb-24">
        {heading}
        {subheading}
      </div>
      <div className="flex flex-row p-6">
        <div className="flex flex-col items-center p-2">
          {"A few lines of text like this"}
          {surveyExample}
          {"Turns into this"}
          <div className="w-1/2 pr-10">
            <RenderedForm
              survey={sampleSurvey}
              mode="test"
              showSubmissionData={false}
            ></RenderedForm>
          </div>
        </div>
      </div>
      {features}
      {benefits}
      {howitworks}
      {faqs}
    </div>
  );
}
