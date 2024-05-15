import React, { useState } from "react";

import { invoke } from "@tauri-apps/api";

import { Input, inputClasses } from '@mui/base/Input';
import { styled } from '@mui/system';
import IconButton from '@mui/material/IconButton';
import SendIcon from '@mui/icons-material/Send';
import Grid from '@mui/material/Grid';


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
        <form onSubmit={handleSubmit} sx={{ width: '100%' }}>
            <Grid container spacing={2}>
                <Grid item>
                    <StyledInput
                        type="text"
                        value={inputValue}
                        onChange={handleInputChange}
                        placeholder="Type something..."
                    />
                </Grid>
                <Grid item>
                    <IconButton type="submit">
                        <SendIcon />
                    </IconButton>
                </Grid>
            </Grid>
        </form>
    );
};

export default InputForm;

const StyledInput = styled(Input)(
    () => `
  display: inline-block;
  .${inputClasses.input} {
    width: 100%;
    font-family: 'IBM Plex Sans', sans-serif;
    font-size: 0.820rem;
    font-weight: 400;
    line-height: 1.5;
    padding: 8px 12px;
    border-radius: 16px;
`,
);