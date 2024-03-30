import {
  Button,
  FormControl,
  FormErrorMessage,
  Input,
  Text,
  VStack,
} from "@chakra-ui/react";
import React from "react";
import { useSignUpForm } from "@/hooks";

interface ISignUp {
  onSubmit: (pw: string) => Promise<void>;
}

export const SignUp: React.FC<ISignUp> = ({ onSubmit }) => {
  const {
    states: { form, isLoading },
    methods: { handleFormSubmit },
  } = useSignUpForm(onSubmit);

  return (
    <form onSubmit={handleFormSubmit}>
      <VStack>
        <Text color="#eee">Create password to save your wallet</Text>
        <FormControl isInvalid={!!form.formState.errors.password}>
          <Input
            type="password"
            placeholder="Password"
            color="white"
            {...form.register("password")}
            isDisabled={isLoading}
          />
          {form.formState.errors.password && (
            <FormErrorMessage>
              {form.formState.errors.password.message}
            </FormErrorMessage>
          )}
        </FormControl>
        <FormControl isInvalid={!!form.formState.errors.confirmPassword}>
          <Input
            type="password"
            placeholder="Confirm password"
            color="white"
            {...form.register("confirmPassword")}
            isDisabled={isLoading}
          />
          {form.formState.errors.confirmPassword && (
            <FormErrorMessage>
              {form.formState.errors.confirmPassword.message}
            </FormErrorMessage>
          )}
        </FormControl>
        <Button
          type="submit"
          variant="solid"
          colorScheme="blue"
          w="100%"
          isDisabled={isLoading}
          isLoading={isLoading}
        >
          Go
        </Button>
      </VStack>
    </form>
  );
};
