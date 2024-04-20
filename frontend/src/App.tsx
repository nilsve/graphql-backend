import './App.css'
import {AllNotes} from './pages/all-notes';
import {Route, Routes} from 'react-router-dom';

function App() {
    return (
        <Routes>
            <Route path="*" element={<AllNotes/>}/>
        </Routes>
    )
}

export default App
