import './App.css'
import { useEffect, useState } from 'react';
import { listAllNotes, Note } from './api/notes';

function App() {
    const [notes, setNotes] = useState<Note[]>([]);

    useEffect(() => {
        (async () => {
            const result = await listAllNotes();
            setNotes(result);
        })();
    }, []);

    return (
        <>
            {
                notes.map((note) => (
                    <div key={note.id}>
                        <h2>{note.title}</h2>
                        <p>{note.body}</p>
                    </div>
                ))
            }
        </>
    )
}

export default App
