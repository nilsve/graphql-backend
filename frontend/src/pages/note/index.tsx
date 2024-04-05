import React from 'react';
import { useParams } from 'react-router-dom';

export const Note: React.FC = () => {
    const noteIdParam = useParams().noteId;

    return <>{noteIdParam}</>
}
