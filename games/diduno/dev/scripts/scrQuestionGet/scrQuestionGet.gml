///@function scrQuestionGet(id, property)
///@description Get the given property of the given question id
///@param id
///@param property
var question = argument0, property = argument1;
switch(question) {
	case 0:
			 if (property == QuestionProperties.Text) { return "The correct answer is option 0."; }
		else if (property == QuestionProperties.Option0) { return "Option 0"; }
		else if (property == QuestionProperties.Option1) { return "Option 1"; }
		else if (property == QuestionProperties.Option2) { return "Option 2"; }
		else if (property == QuestionProperties.Option3) { return "Option 3"; }
		else if (property == QuestionProperties.CorrectOption) { return 0; }
	break;
	case 1:
			 if (property == QuestionProperties.Text) { return "The correct answer is option 1."; }
		else if (property == QuestionProperties.Option0) { return "Option 0"; }
		else if (property == QuestionProperties.Option1) { return "Option 1"; }
		else if (property == QuestionProperties.Option2) { return "Option 2"; }
		else if (property == QuestionProperties.Option3) { return "Option 3"; }
		else if (property == QuestionProperties.CorrectOption) { return 1; }
	break;
	case 2:
			 if (property == QuestionProperties.Text) { return "The correct answer is option 2."; }
		else if (property == QuestionProperties.Option0) { return "Option 0"; }
		else if (property == QuestionProperties.Option1) { return "Option 1"; }
		else if (property == QuestionProperties.Option2) { return "Option 2"; }
		else if (property == QuestionProperties.Option3) { return "Option 3"; }
		else if (property == QuestionProperties.CorrectOption) { return 2; }
	break;
	case 3:
			 if (property == QuestionProperties.Text) { return "The correct answer is option 3."; }
		else if (property == QuestionProperties.Option0) { return "Option 0"; }
		else if (property == QuestionProperties.Option1) { return "Option 1"; }
		else if (property == QuestionProperties.Option2) { return "Option 2"; }
		else if (property == QuestionProperties.Option3) { return "Option 3"; }
		else if (property == QuestionProperties.CorrectOption) { return 3; }
	break;
}