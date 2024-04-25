import React from 'react';
import { Box, Link, Text } from '@chakra-ui/react';
import { Link as RouterLink } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';
import ChakraUIRenderer from 'chakra-ui-markdown-renderer';
import { Note } from '../../../api/notes';

interface Props {
    note: Note;
}

export const NotePreview: React.FC<Props> = ({ note }) => {
    return <Link as={RouterLink} to={`/${note.id}`} textDecoration="none">
        <Box p={4} boxShadow="md" borderRadius="md" bg="white">
            <Text fontSize="lg" fontWeight="bold" mb={2}>
                {note.title}
            </Text>
            <ReactMarkdown components={ChakraUIRenderer()}>{note.body}</ReactMarkdown>
        </Box>
    </Link>
}
