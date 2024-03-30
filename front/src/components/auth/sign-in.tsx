import {
  Button,
  FormControl,
  FormErrorMessage,
  HStack,
  Input,
} from "@chakra-ui/react";
import React from "react";
import { useLoginForm } from "../../hooks";

interface ISignIn {
  onSubmit: (pw: string) => Promise<void>;
}

export const SignIn: React.FC<ISignIn> = ({ onSubmit }) => {
  const {
    states: { form, isLoading },
    methods: { handleFormSubmit },
  } = useLoginForm(onSubmit);

  return (
    <form onSubmit={handleFormSubmit}>
      <FormControl isInvalid={!!form.formState.errors.password}>
        <HStack>
          <Input
            type="password"
            placeholder="Password"
            color="white"
            {...form.register("password")}
            isDisabled={isLoading}
          />
          <Button
            type="submit"
            variant="solid"
            colorScheme="blue"
            w="120px"
            isLoading={isLoading}
            isDisabled={isLoading}
          >
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
