pipeline {
	agent any
		stages {
			stage('Checking') {
				steps {
					sh '''
						. ~/.cargo/env
						cargo clippy
						'''
				}
			}
			stage('Build') {
				steps {
					sh '''
						. ~/.cargo/env
						cargo build
						'''
				}
			}
			stage('Test') {
				steps {
					sh '''
						. ~/.cargo/env
						cargo test --release
						'''
				}
			}
		}
	post {
		unsuccessful {
			emailextrecipients([culprits()])
		}
		always {
			updateGithubCommitStatus(currentBuild)
		}
	}
}

def getRepoURL() {
	sh "git config --get remote.origin.url > .git/remote-url"
		return readFile(".git/remote-url").trim()
}

def getCommitSha() {
	sh "git rev-parse HEAD > .git/current-commit"
		return readFile(".git/current-commit").trim()
}

def updateGithubCommitStatus(build) {
	// workaround https://issues.jenkins-ci.org/browse/JENKINS-38674
	repoUrl = getRepoURL()
		commitSha = getCommitSha()

		step([
				$class: 'GitHubCommitStatusSetter',
				reposSource: [$class: "ManuallyEnteredRepositorySource", url: repoUrl],
				commitShaSource: [$class: "ManuallyEnteredShaSource", sha: commitSha],
				errorHandlers: [[$class: 'ShallowAnyErrorHandler']],
				statusResultSource: [
				$class: 'ConditionalStatusResultSource',
				results: [
				[$class: 'BetterThanOrEqualBuildResult', result: 'SUCCESS', state: 'SUCCESS', message: build.description],
				[$class: 'BetterThanOrEqualBuildResult', result: 'FAILURE', state: 'FAILURE', message: build.description],
				[$class: 'AnyBuildResult', state: 'FAILURE', message: 'Loophole']
				]
				]
		])
}
