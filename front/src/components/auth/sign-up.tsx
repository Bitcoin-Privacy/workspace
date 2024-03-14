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

export const SignUp: React.FC<{}> = () => {
  const {
    states: { form },
    methods: { handleFormSubmit },
  } = useSignUpForm();

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
          />
          {form.formState.errors.confirmPassword && (
            <FormErrorMessage>
              {form.formState.errors.confirmPassword.message}
            </FormErrorMessage>
          )}
        </FormControl>
        <Button type="submit" variant="solid" colorScheme="blue" w="100%">
          Go
        </Button>
      </VStack>
    </form>
  );
};
