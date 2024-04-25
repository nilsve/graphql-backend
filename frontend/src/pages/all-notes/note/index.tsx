import React, { ChangeEvent, useCallback, useState } from 'react';
import { Box, Button, Card, FormControl, Input, Switch, Text } from '@chakra-ui/react';
import { Note } from '../../../api/notes';
import { SubmitHandler, useForm } from 'react-hook-form';
import { AutoResizeTextarea } from '../../../helpers/auto-resize-textarea.tsx';
import ChakraUIRenderer from 'chakra-ui-markdown-renderer';
import ReactMarkdown from 'react-markdown';

interface Props {
    note: Note;
    editData: {
        onNoteUpdate: SubmitHandler<Note>;
        onCancel: () => unknown;
    }
}

export const NoteComponent: React.FC<Props> = ({ note, editData }) => {
    const [isEditMode, setIsEditMode] = useState(!!editData);

    if (isEditMode && !editData) {
        throw new Error('editData is required when isEditMode is true');
    }

    const { register, handleSubmit, getValues } = useForm({
        values: note
    });

    const handleChangeIsEditMode = useCallback((event: ChangeEvent<HTMLInputElement>) => {
        setIsEditMode(event.target.checked)
    }, []);

    return (
        <Card>
            <Box h={'full'} p={4} boxShadow="md" borderRadius="md" bg="white">
                <form onSubmit={handleSubmit(editData.onNoteUpdate)}>
                    <FormControl isRequired>
                        <Input autoFocus={true} placeholder={'Title'} {...register('title')} />
                    </FormControl>
                    <FormControl textAlign={'left'}>
                        <Switch mt={6} isChecked={isEditMode} onChange={handleChangeIsEditMode}>Edit
                            mode</Switch>
                    </FormControl>
                    <FormControl>
                        {
                            isEditMode ? (
                                <AutoResizeTextarea mt={4} {...register('body')}
                                                    placeholder={'Your interesting notes...'} />
                            ) : (
                                <Text mt={4}>
                                    <ReactMarkdown components={ChakraUIRenderer()}>{getValues('body')}</ReactMarkdown>
                                </Text>
                            )
                        }
                    </FormControl>
                    <Box display={'flex'} flexDirection={'row'}>
                        <FormControl textAlign={'right'}>
                            <Button mt={4} mr={4} type={'submit'}>Save</Button>
                            <Button mt={4} onClick={editData.onCancel}>Cancel</Button>
                        </FormControl>
                    </Box>

                </form>
            </Box>
        </Card>
    );
}
