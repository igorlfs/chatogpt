import React from 'react';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import AccountCircleIcon from '@mui/icons-material/AccountCircle';
import ChatBubbleIcon from '@mui/icons-material/ChatBubble';
import InputForm from './InputForm';

const Chat = () => {
    const [messages, setMessages] = React.useState([]);

    const handleSendMessage = async (message) => {
        // Adiciona a mensagem escrita no chat
        setMessages(prevMessages => [...prevMessages, { id: prevMessages.length + 1, text: message }]);
        
        // Adiciona uma pausa de 1 segundo antes de enviar a resposta
        await new Promise(resolve => setTimeout(resolve, 1000));

        // Adiciona a mensagem de resposta como uma nova mensagem com ícone diferente
        setMessages(prevMessages => [
            ...prevMessages, 
            { id: prevMessages.length + 2, text: 'Dio!', icon: <AccountCircleIcon color="secondary" /> }
        ]);
    };

    return (
        <Box sx={{ pb: 7 }}>
            <CssBaseline />
            {messages.map((msg) => (
                <Box key={msg.id} sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                    {msg.icon ? msg.icon : <ChatBubbleIcon />}
                    <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                        {msg.text}
                    </Typography>
                </Box>
            ))}
            <Paper sx={{ display: 'flex', position: 'fixed', bottom: 0, left: 0, right: 0, height: '80px', padding: '2%', justifyContent: 'center' }} elevation={3}>
                <InputForm onSendMessage={handleSendMessage} />
            </Paper>
        </Box>
    );
};

export default Chat;
