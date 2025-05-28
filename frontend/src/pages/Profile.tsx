import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const Profile: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Profile
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Manage your profile here</Typography>
      </Paper>
    </Container>
  );
};

export default Profile;
