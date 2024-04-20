import React from 'react';
import {Link as RouterLink} from 'react-router-dom';
import {Box, Button, Card, FormControl, Input, Link, Text} from "@chakra-ui/react";
import {Note} from "../../../api/notes";
import {SubmitHandler, useForm} from "react-hook-form";
import {AutoResizeTextarea} from "../../../helpers/auto-resize-textarea.tsx";
import ChakraUIRenderer from 'chakra-ui-markdown-renderer';
import ReactMarkdown from "react-markdown";

interface Props {
    note: Note;
    editData?: {
        onNoteUpdate: SubmitHandler<Note>;
        onCancel: () => unknown;
    }

}

export const NoteComponent: React.FC<Props> = ({note, editData}) => {
    const isEditMode = !!editData;

    const {register, handleSubmit} = useForm({
        values: note,
    });

    const body = isEditMode ? (
        <Box h={'full'} p={4} boxShadow="md" borderRadius="md" bg="white">
            <form onSubmit={handleSubmit(editData.onNoteUpdate)}>
                <FormControl isRequired>
                    <Input autoFocus={true} placeholder={'Title'} {...register('title')} />
                </FormControl>
                <FormControl>
                    <AutoResizeTextarea mt={4} {...register('body')}
                                        placeholder={'Your interesting notes...'}/>
                </FormControl>
                <Button mt={4} type={'submit'}>Save</Button>
                <Button mt={4} onClick={editData.onCancel}>Cancel</Button>
            </form>
        </Box>
    ) : (
        <Link as={RouterLink} to={`/${note.id}`} textDecoration="none">
            <Box p={4} boxShadow="md" borderRadius="md" bg="white">
                <Text fontSize="lg" fontWeight="bold" mb={2}>
                    {note.title}
                </Text>
                <ReactMarkdown components={ChakraUIRenderer()}>{note.body}</ReactMarkdown>
            </Box>
        </Link>
    );

    return (
        <Card>
            {body}
        </Card>
    );
}
