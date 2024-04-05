import React, { useEffect, useState } from 'react';
import { listAllNotes, Note } from '../../api/notes';
import { Box, Button } from '@chakra-ui/react';

export const AllNotes: React.FC = () => {
    const [notes, setNotes] = useState<Note[]>([]);

    useEffect(() => {
        (async () => {
            const result = await listAllNotes();
            setNotes(result);
        })();
    }, []);

    return (
        <Box>
            {
                notes.map((note) => (
                    <div key={note.id}>
                        <Button>View</Button>
                        <h2>{note.title}</h2>
                        <p>{note.body}</p>
                    </div>
                ))
            }
        </Box>
    )
}
