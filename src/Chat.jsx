import * as React from 'react';
import Box from '@mui/material/Box';
import CssBaseline from '@mui/material/CssBaseline';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';

import AccountCircleIcon from '@mui/icons-material/AccountCircle';
import ChatBubbleIcon from '@mui/icons-material/ChatBubble';

import InputForm from './InputForm';

export default function Chat() {
    return (
        <Box sx={{ pb: 7 }}>
            <CssBaseline />
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <AccountCircleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <ChatBubbleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <AccountCircleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <ChatBubbleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <AccountCircleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, padding: '2%' }}>
                <ChatBubbleIcon />
                <Typography variant="body2" sx={{ fontSize: '0.875rem' }}>
                    Lorem ipsum dolor sit amet, consectetur adipisicing elit. Assumenda veritatis odio soluta, nihil cum, rem vitae, modi repellat voluptate odit earum explicabo.
                </Typography>
            </Box>
            <Paper sx={{ display: 'flex', position: 'fixed', bottom: 0, left: 0, right: 0, height: '80px', padding: '2%', justifyContent: 'center' }} elevation={3}>
                <InputForm />
            </Paper>
        </Box>
    );
}

