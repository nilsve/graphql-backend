import {useNavigate, useParams} from 'react-router-dom';
import React, {useCallback, useEffect, useState} from 'react';
import {deleteNote, getNote, Note, updateNote} from '../../../api/notes';
import {NoteComponent} from '../note';
import {SubmitHandler} from 'react-hook-form';

export const EditNote: React.FC = () => {
    const {noteId} = useParams<{ noteId: string }>();

    const navigate = useNavigate();

    if (!noteId) {
        return <div>Invalid note id</div>;
    }

    const [note, setNote] = useState<Note | null>(null);

    useEffect(() => {
        (async () => {
            const result = await getNote(noteId);
            setNote(result);
        })();
    }, [noteId, setNote]);

    const handleUpdateNote: SubmitHandler<Note> = useCallback(async (note: Note) => {
        await updateNote(note);
        navigate('/', {
            replace: true
        });
    }, [navigate]);

    const handleDeleteNote: SubmitHandler<Note> = useCallback(async (note: Note) => {
        await deleteNote(note);
        navigate('/', {
            replace: true,
        })
    }, [navigate]);

    const handleCancel = useCallback(() => {
        navigate('/', {
            replace: true
        });
    }, []);

    if (!note) {
        return <div>Loading...</div>;
    }

    return (
        <NoteComponent note={note} editData={{
            onNoteUpdate: handleUpdateNote,
            onCancel: handleCancel,
            onNoteDelete: handleDeleteNote
        }}/>
    );
}
