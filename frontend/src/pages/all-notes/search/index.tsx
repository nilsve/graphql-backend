import React from "react";
import {Box, Input} from "@chakra-ui/react";
import {SubmitHandler, useForm} from "react-hook-form";

export interface SearchForm {
    searchText: string;
}

interface Props {
    onSearch: SubmitHandler<SearchForm>;
}

export const SearchNoteComponent: React.FC<Props> = ({onSearch}) => {
    const {register, handleSubmit} = useForm<SearchForm>({
        defaultValues: {
            searchText: ''
        }
    });

    return (
        <Box mb={4}>
            <form onSubmit={handleSubmit(onSearch)}>
                <Input placeholder='Search Notes' {...register('searchText')}/>
            </form>
        </Box>
    );
}