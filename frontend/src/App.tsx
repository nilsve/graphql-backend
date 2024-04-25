import './App.css'
import { AllNotes } from './pages/all-notes';
import { Route, Routes } from 'react-router-dom';
import { EditNote } from './pages/all-notes/edit-note/edit-note.tsx';
import { Box } from '@chakra-ui/react';

function App() {
    return (
        <Box w={'90%'} m={'auto'} p={'1em'}>
            <Routes>
                <Route path="/" element={<AllNotes />} />
                <Route path=":noteId" element={<EditNote />} />
            </Routes>
        </Box>
    )
}

export default App;
