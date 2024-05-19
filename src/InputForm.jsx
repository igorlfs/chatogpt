// InputForm.jsx
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { Input, inputClasses } from '@mui/base/Input';
import { styled } from '@mui/system';
import IconButton from '@mui/material/IconButton';
import SendIcon from '@mui/icons-material/Send';
import Grid from '@mui/material/Grid';

const StyledInput = styled(Input)(
    () => `
  display: block;
  width: 100%;
  .${inputClasses.input} {
    width: 100%;
    font-family: 'IBM Plex Sans', sans-serif;
    font-size: 0.820rem;
    font-weight: 400;
    line-height: 1.5;
    padding: 8px 12px;
    border-radius: 16px;
    border: 1px solid #ccc; /* Optional: add a border for better visualization */
  }
`,
);

const InputForm = ({ onSendMessage }) => {
    const [inputValue, setInputValue] = useState("");

    const handleInputChange = (event) => {
        setInputValue(event.target.value);
    };

    const handleSubmit = async (event) => {
        event.preventDefault();
        await submitMessage();
    };

    const handleButtonClick = async () => {
        console.log("Botão clicado"); // Imprime no terminal quando o botão é clicado
        await submitMessage();
    };

    const submitMessage = async () => {
        if (window.__TAURI_IPC__) {
            try {
                const [_, reply] = await invoke("message_to_reply", {
                    message: inputValue,
                    threadId: 0,
                });
                console.log("Submitted value:", reply);
                onSendMessage(reply);
            } catch (error) {
                console.error("Error invoking Tauri command:", error);
            }
        } else {
            console.log("Tauri não está disponível. Imprimindo mensagem no console.");
            console.log("Submitted value:", inputValue);
            onSendMessage(inputValue);
        }
        setInputValue("");
    };

    return (
        <form onSubmit={handleSubmit} style={{ width: '70%' }}>
            <Grid container spacing={2} alignItems="center">
                <Grid item xs={10}>
                    <StyledInput
                        type="text"
                        value={inputValue}
                        onChange={handleInputChange}
                        placeholder="Type something..."
                    />
                </Grid>
                <Grid item xs={2}>
                    <IconButton type="button" onClick={handleButtonClick}>
                        <SendIcon />
                    </IconButton>
                </Grid>
            </Grid>
        </form>
    );
};

export default InputForm;
