import './App.css'
import { AllNotes } from './pages/all-notes';
import { Route, Routes } from 'react-router-dom';
import { Note } from './pages/note';

function App() {
    return (
        <Routes>
            <Route path="" element={<AllNotes />} />
            <Route path=":noteId" element={<Note />} />
        </Routes>
    )
}

export default App
