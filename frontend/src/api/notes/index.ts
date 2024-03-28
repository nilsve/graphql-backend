const API_URL = 'http://localhost:8080';//import.meta.env.API_URL;

export interface Note {
    id: string;
    title: string;
    body: string;
}

export interface NewNote {
    title: string;
    content: string;
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
