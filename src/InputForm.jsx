import React, { useState } from "react";

import { invoke } from "@tauri-apps/api";

const InputForm = () => {
    const [inputValue, setInputValue] = useState("");

    const handleInputChange = (event) => {
        setInputValue(event.target.value);
    };

    const handleSubmit = async (event) => {
        event.preventDefault();
        const [_, reply] = await invoke("message_to_reply", {
            message: inputValue,
            threadId: 0,
        });
        console.log("Submitted value:", reply);
        setInputValue("");
    };

    return (
        <form onSubmit={handleSubmit}>
            <input
                type="text"
                value={inputValue}
                onChange={handleInputChange}
                placeholder="Type something..."
            />
            <button type="submit">Send</button>
        </form>
    );
};

export default InputForm;
