import React, {useCallback, useState} from "react";
import {Box, Input, Card} from "@chakra-ui/react";
import {SubmitHandler, useForm} from "react-hook-form";
import {askQuestion} from "../../../api/notes";
import ReactMarkdown from "react-markdown";
import ChakraUIRenderer from "chakra-ui-markdown-renderer";

interface QuestionForm {
    question: ''
}

export const SearchNoteComponent: React.FC = () => {
    const [answer, setAnswer] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const {register, handleSubmit} = useForm<QuestionForm>({
        defaultValues: {
            question: ''
        }
    });

    const onSearch: SubmitHandler<QuestionForm> = useCallback(async ({question}) => {
        setIsLoading(true);
        setAnswer('');
        try {
            const response = await askQuestion(question);
            setAnswer(response.answer);

        } finally {
            setIsLoading(false);
        }
    }, [setIsLoading, setAnswer]);

    return (
        <Box mb={4}>
            <form onSubmit={handleSubmit(onSearch)}>
                <Input placeholder='Ask a question' {...register('question')}/>
            </form>

            {
                isLoading && (
                    <Box mt={4}>
                        Loading...
                    </Box>
                )
            }
            {
                answer && (
                    <Box mt={4} textAlign={'center'}>
                        <Card p={4}>
                            <ReactMarkdown components={ChakraUIRenderer()}>{answer}</ReactMarkdown>
                        </Card>
                    </Box>
                )
            }

        </Box>
    );
}