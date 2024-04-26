import React, {useEffect, useState} from 'react';
import {listAllNotes, Note} from '../../api/notes';
import {
    Box,
    Grid,
    GridItem
} from '@chakra-ui/react';
import {SearchNoteComponent} from './search';
import {CreateNote} from './create-note/create-note.tsx';
import {NotePreview} from './note-preview';

export const AllNotes: React.FC = () => {
    const [notes, setNotes] = useState<Note[]>([]);

    useEffect(() => {
        (async () => {
            const result = await listAllNotes();
            setNotes(result);
        })();
    }, []);

    return (
        <Box w={'100%'}>
            <SearchNoteComponent/>

            <CreateNote/>
            <Grid templateColumns="repeat(auto-fill, minmax(250px, 1fr))" gap={6}>
                {
                    notes.map((note) => {
                        return (
                            <GridItem key={note.id}>
                                <NotePreview note={note}/>
                            </GridItem>
                        );
                    })
                }
            </Grid>
        </Box>
    )
}
