import {useNavigate, useParams} from "react-router-dom";
import React, {useCallback, useEffect, useState} from "react";
import {getNote, Note, updateNote} from "../../../api/notes";
import {NoteComponent} from "../note";
import {SubmitHandler} from "react-hook-form";
import {Modal, ModalContent, ModalOverlay} from "@chakra-ui/react";

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
    }, [noteId]);

    const handleUpdateNote: SubmitHandler<Note> = useCallback(async (note: Note) => {
        await updateNote(note);
        navigate('/', {
            replace: true,
        });
    }, [updateNote]);

    const handleCancel = useCallback(() => {
        navigate('/', {
            replace: true,
        });
    }, []);

    if (!note) {
        return <div>Loading...</div>;
    }

    return (
        <Modal size={'xxl'} isOpen={true} onClose={handleCancel}>
            <ModalOverlay/>
            <ModalContent>
                <NoteComponent note={note} editData={{onNoteUpdate: handleUpdateNote, onCancel: handleCancel}}/>
            </ModalContent>
        </Modal>
    );
}