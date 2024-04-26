import React, {useCallback, useState} from "react";
import {Box, Input, Card, Text} from "@chakra-ui/react";
import {SubmitHandler, useForm} from "react-hook-form";
import {askQuestion} from "../../../api/notes";

interface QuestionForm {
    question: ''
}

export const SearchNoteComponent: React.FC = () => {
    const [answer, setAnswer] = useState('');
    const {register, handleSubmit} = useForm<QuestionForm>({
        defaultValues: {
            question: ''
        }
    });

    const onSearch: SubmitHandler<QuestionForm> = useCallback(async ({question}) => {
        const response = await askQuestion(question);

        setAnswer(response.answer);
    }, []);

    return (
        <Box mb={4}>
            <form onSubmit={handleSubmit(onSearch)}>
                <Input placeholder='Ask a question' {...register('question')}/>
            </form>

            {
                answer && (
                    <Box mt={4} textAlign={'center'}>
                        <Card p={4}><Text>{answer}</Text></Card>
                    </Box>
                )
            }

        </Box>
    );
}