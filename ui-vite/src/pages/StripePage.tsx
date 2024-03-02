import React, { useState, useEffect } from "react";
// import "./App.css";

const ProductDisplay = () => (
    <section className="w-2/3 bg-yellow gap-2 p-4">
        <div className="flex flex-row p-2">
            <img
                src="https://i.imgur.com/EHyR2nP.png"
                alt="The cover of Stubborn Attachments"
                className="w-32 h-32 aspect-square"
            />
            <div className="w-full h-full text-left p-2 items-center">
                <h3>Stubborn Attachments</h3>
                <h5>$20.00</h5>
            </div>
        </div>
        <form action="/create-checkout-session" method="POST">
            <button className="bg-pink w-full rounded-md" type="submit">
                Checkout
            </button>
        </form>
    </section>
);

const Message = ({ message }) => (
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

    return message ? (
        <Message message={message} />
    ) : (
        <ProductDisplay />
    );
}