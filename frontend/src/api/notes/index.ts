const API_URL = '/api';//import.meta.env.API_URL;

export interface Note {
    id: string;
    title: string;
    body: string;
}

export interface NewNote {
    title: string;
    body: string;
}

export const createNote = (note: NewNote): Promise<Note> => {
    return fetch(`${API_URL}/notes`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(note)
    }).then((response) => response.json());
}

export const deleteNote = (note: Note): Promise<void> => {
    return fetch(`${API_URL}/notes/${note.id}`, {
        method: 'DELETE'
    }).then(() => {
    });
}

export const updateNote = (note: Note): Promise<Note> => {
    return fetch(`${API_URL}/notes/${note.id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(note)
    }).then((response) => response.json());
}

export const listAllNotes = (): Promise<Note[]> => {
    return fetch(`${API_URL}/notes`, {
        method: 'GET'
    }).then((response) => response.json());
}

export const getNote = (id: string): Promise<Note> => {
    return fetch(`${API_URL}/notes/${id}`, {
        method: 'GET'
    }).then((response) => response.json());
}

interface Answer {
    answer: string;
}

export const askQuestion = async (question: string): Promise<Answer> => {
    const response = await fetch(`${API_URL}/ai`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            question
        })
    });

    return response.json();
}
