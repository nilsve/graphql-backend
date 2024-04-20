import React, {useCallback} from "react";
import {createNote, Note} from "../../../api/notes";
import {NoteComponent} from "../note";
import {SubmitHandler} from "react-hook-form";
import {Input, Modal, ModalContent, ModalOverlay} from "@chakra-ui/react";

export const CreateNote: React.FC = () => {
    const [isOpen, setIsOpen] = React.useState(false);


    const handleCreateNote: SubmitHandler<Note> = useCallback(async (note) => {
        await createNote({
            body: note.body,
            title: note.title
        });

        setIsOpen(false);
    }, []);

    const handleCancel = useCallback(() => {
        setIsOpen(false);
    }, []);

    return (
        <>
            {!isOpen && <Input mb={4} placeholder='Create Note' onClick={() => setIsOpen(true)}/>}
            <Modal size={'full'} isOpen={isOpen} onClose={() => setIsOpen(false)}>
                <ModalOverlay/>
                <ModalContent>
                    <NoteComponent note={{id: '', title: '', body: ''}}
                                   editData={{onNoteUpdate: handleCreateNote, onCancel: handleCancel}}/>
                </ModalContent>
            </Modal>
        </>
    )
}