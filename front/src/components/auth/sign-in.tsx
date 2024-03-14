import {
  Button,
  FormControl,
  FormErrorMessage,
  HStack,
  Input,
} from "@chakra-ui/react";
import React from "react";
import { useLoginForm } from "../../hooks";
import { InitStateEnum } from "@/dtos";

export const SignIn: React.FC<{ state: InitStateEnum; password: string }> = ({
  password,
  state,
}) => {
  const {
    states: { form },
    methods: { handleFormSubmit },
  } = useLoginForm(password, state);

  return (
    <form onSubmit={handleFormSubmit}>
      <FormControl isInvalid={!!form.formState.errors.password}>
        <HStack>
          <Input
            type="password"
            placeholder="Password"
            color="white"
            {...form.register("password")}
          />
          <Button type="submit" variant="solid" colorScheme="blue" w="120px">
            Go
          </Button>
        </HStack>
        {form.formState.errors.password && (
          <FormErrorMessage>
            {form.formState.errors.password.message}
          </FormErrorMessage>
        )}
      </FormControl>
    </form>
  );
};
