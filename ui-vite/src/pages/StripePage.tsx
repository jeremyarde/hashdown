import { getApiBaseUrl, getSessionToken, getWebsiteUrl } from "@/lib/utils";
import React, { useState, useEffect } from "react";

const createCheckoutSession = async () => {
  const response = await fetch(
    `${getApiBaseUrl()}/v1/create-checkout-session`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        session_id: getSessionToken(),
      },
      body: JSON.stringify({
        price_id: "price_1I1w6lI5j0q7u0x7x0",
        success_url: `${getWebsiteUrl()}?success=true`,
        cancel_url: `${getWebsiteUrl()}?canceled=true`,
      }),
    }
  );
  console.log(`createCheckoutSession response: ${JSON.stringify(response)}`);
};

const ProductDisplay = () => (
  <section className="w-2/3 gap-2 p-4 bg-yellow">
    <div className="flex flex-row p-2">
      <img
        src="https://i.imgur.com/EHyR2nP.png"
        alt="The cover of Stubborn Attachments"
        className="w-32 h-32 aspect-square"
      />
      <div className="items-center w-full h-full p-2 text-left">
        <h3>Stubborn Attachments</h3>
        <h5>$20.00</h5>
      </div>
    </div>
    <form onSubmit={createCheckoutSession}>
      <button className="w-full rounded-md bg-pink" type="submit">
        Checkout
      </button>
    </form>
  </section>
);

const Message = ({ message }: { message: string }) => (
  <section>
    <p>{message}</p>
  </section>
);

export default function StripePage() {
  const [message, setMessage] = useState("");

  useEffect(() => {
    // Check to see if this is a redirect back from Checkout
    const query = new URLSearchParams(window.location.search);

    if (query.get("success")) {
      setMessage("Order placed! You will receive an email confirmation.");
    }

    if (query.get("canceled")) {
      setMessage(
        "Order canceled -- continue to shop around and checkout when you're ready."
      );
    }
  }, []);

  return message ? <Message message={message} /> : <ProductDisplay />;
}
